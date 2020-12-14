//! # Me
//! Me module.

extern crate reqwest;
extern crate serde_json;

use crate::{
    client::{Config, Reddit},
    subreddit::responses::Submissions,
    util::RedditError,
};

pub mod responses;
use responses::MeData;

/// Me
pub struct Me<'a> {
    client: &'a Reddit,
}

impl<'a> Me<'a> {
    pub fn create_new(client: &'a Reddit) -> Me<'a> {
        Me { client }
    }

    /// Get me
    pub async fn me(&self) -> Result<MeData, RedditError> {
        let url = format!("https://oauth.reddit.com/api/v1/me{}", "");

        Ok(self.client.get(&url).send().await?.json::<MeData>().await?)
    }

    // /// Submit link
    // pub async fn submit_link(
    //     &self,
    //     title: &str,
    //     link: &str,
    //     sr: &str,
    // ) -> Result<Response, RedditError> {
    //     let form = [
    //         ("kind", "link"),
    //         ("title", title),
    //         ("url", link),
    //         ("sr", sr),
    //     ];

    //     self.post("api/submit", &form).await
    // }

    // /// Submit text
    // pub async fn submit_text(
    //     &self,
    //     title: &str,
    //     text: &str,
    //     sr: &str,
    // ) -> Result<Response, RedditError> {
    //     let form = [
    //         ("kind", "self"),
    //         ("title", title),
    //         ("text", text),
    //         ("sr", sr),
    //     ];

    //     self.post("api/submit", &form).await
    // }

    // /// Compose message
    // pub async fn compose_message(
    //     &self,
    //     username: &str,
    //     subject: &str,
    //     body: &str,
    // ) -> Result<Response, RedditError> {
    //     let form = [
    //         ("api_type", "json"),
    //         ("subject", subject),
    //         ("text", body),
    //         ("to", username),
    //     ];

    //     self.post("api/compose", &form).await
    // }

    // /// Get user's submitted posts.
    // pub async fn inbox(&self) -> Result<Inbox, RedditError> {
    //     Ok(self.get("message/inbox").await?.json::<Inbox>().await?)
    // }

    // /// Get saved
    // pub async fn saved(&self) -> Result<Submissions, RedditError> {
    //     let url = format!("https://oauth.reddit.com/user/rickhuis/saved/.json");
    //
    //     let result = self.client.get(&url).send().await?.text().await?;
    //
    //     println!("{}", result);
    //
    //     Ok(self
    //         .client
    //         .get(&url)
    //         .send()
    //         .await?
    //         .json::<Submissions>()
    //         .await?)
    // }

    // /// Get upvoted
    // pub async fn upvoted(&self) -> Result<Submissions, RedditError> {
    //     let url = format!(
    //         "user/{}/upvoted/.json",
    //         self.config.username.to_owned().unwrap()
    //     );

    //     Ok(self.get(&url).await?.json::<Submissions>().await?)
    // }

    // /// Get downvoted
    // pub async fn downvoted(&self) -> Result<Submissions, RedditError> {
    //     let url = format!(
    //         "user/{}/downvoted/.json",
    //         self.config.username.to_owned().unwrap()
    //     );

    //     Ok(self.get(&url).await?.json::<Submissions>().await?)
    // }

    // /// Get users unread messages
    // pub async fn unread(&self) -> Result<Inbox, RedditError> {
    //     Ok(self.get("message/unread").await?.json::<Inbox>().await?)
    // }

    // /// Mark messages as read
    // pub async fn mark_read(&self, ids: &str) -> Result<Response, RedditError> {
    //     let form = [("id", ids)];
    //     self.post("api/read_message", &form).await
    // }

    // /// Mark messages as unread
    // pub async fn mark_unread(&self, ids: &str) -> Result<Response, RedditError> {
    //     let form = [("id", ids)];
    //     self.post("api/unread_message", &form).await
    // }

    // /// Comment
    // pub async fn comment(&self, text: &str, parent: &str) -> Result<Response, RedditError> {
    //     let form = [("text", text), ("parent", parent)];
    //     self.post("api/comment", &form).await
    // }

    // /// Edit
    // pub async fn edit(&self, text: &str, parent: &str) -> Result<Response, RedditError> {
    //     let form = [("text", text), ("thing_id", parent)];
    //     self.post("api/editusertext", &form).await
    // }

    // /// Logout
    // pub async fn logout(self) -> Result<(), RedditError> {
    //     let url = "https://www.reddit.com/api/v1/revoke_token";

    //     let form = [("access_token", self.access_token.to_owned())];

    //     let response = self
    //         .client
    //         .post(url)
    //         .basic_auth(&self.config.client_id, Some(&self.config.client_secret))
    //         .form(&form)
    //         .send()
    //         .await?;

    //     if response.status() == 204 {
    //         Ok(())
    //     } else {
    //         Err(RedditError::Status(response))
    //     }
    // }
}
