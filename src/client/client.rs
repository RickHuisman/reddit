use reqwest::Client;
use reqwest::RequestBuilder;

use crate::{Me, User, Subreddit};

use super::Config;

pub struct Reddit {
    pub config: Config,
    pub client: Client,
}

impl Reddit {
    pub fn new(config: Config) -> Reddit {
        Reddit {
            config,
            client: Client::new(),
        }
    }

    pub fn get(&self, url: &str) -> RequestBuilder {
        let req = self.client.get(url).headers(self.config.get_headers());
        req
    }

    pub fn post(&self, url: &str) -> RequestBuilder {
        let req = self.client.post(url).headers(self.config.get_headers());
        req
    }

    pub fn me(&self) -> Me {
        Me::new(self)
    }

    pub fn user(&self, name: &str) -> User {
        User::new(self, name)
    }

    pub fn subreddit(&self, name: &str) -> Subreddit {
        Subreddit::create_new(self, name)
    }
}
