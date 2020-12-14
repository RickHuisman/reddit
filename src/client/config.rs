use reqwest::header::{HeaderMap, AUTHORIZATION, USER_AGENT};
use reqwest::Client;
use serde::Deserialize;

use crate::util;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub user_agent: String,
    pub client_id: String,
    pub client_secret: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub auth_data: Option<AuthData>,
}

#[derive(Deserialize, Debug)]
pub struct AuthData {
    pub access_token: String,
}

impl Config {
    pub fn new(user_agent: &str, client_id: &str, client_secret: &str) -> Config {
        Config {
            user_agent: user_agent.to_owned(),
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            username: None,
            password: None,
            auth_data: None,
        }
    }

    pub fn username(mut self, username: &str) -> Config {
        self.username = Some(username.to_owned());
        self
    }

    pub fn password(mut self, password: &str) -> Config {
        self.password = Some(password.to_owned());
        self
    }

    pub fn update_auth(mut self, auth_data: AuthData) -> Config {
        self.auth_data = Some(auth_data);
        self
    }

    pub fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.append(USER_AGENT, self.user_agent[..].parse().unwrap());
        if let Some(auth) = &self.auth_data {
            headers.append(
                AUTHORIZATION,
                format!("Bearer {}", auth.access_token).parse().unwrap(),
            );
        }
        headers
    }

    /// Login as a user.
    pub async fn login(self) -> Result<Config, util::RedditError> {
        let client = Client::new();
        let url = format!("https://www.reddit.com/{}/.json", "api/v1/access_token");

        let form = [
            ("grant_type", "password"),
            ("username", &self.username.to_owned().unwrap()),
            ("password", &self.password.to_owned().unwrap()),
        ];

        let request = client
            .post(url.as_str())
            .header(USER_AGENT, &self.user_agent[..])
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&form);

        let response = request.send().await?;

        if response.status() == 200 {
            let auth_data = response.json::<AuthData>().await.unwrap();
            Ok(self.update_auth(auth_data))
        } else {
            Err(util::RedditError::Status(response))
        }
    }
}
