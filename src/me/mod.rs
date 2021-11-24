//! Me module.

use crate::{client::Reddit, subreddit::responses::Submissions};

use crate::{
    util::{
        builder::ListingRequestBuilder, RedditError,
        TimeFilter,
    },
};
pub mod responses;
use responses::Inbox;
use responses::MeData;


/// Me
pub struct Me<'a> {
    client: &'a Reddit,
}

impl<'a> Me<'a> {
    pub fn new(client: &'a Reddit) -> Me<'a> {
        Me { client }
    }

    /// Get me
    pub async fn me(&self) -> Result<MeData, RedditError> {
        let url = format!("https://oauth.reddit.com/api/v1/me");

        Ok(self.client.get(&url).send().await?.json::<MeData>().await?)
    }

    /// Get users inbox
    pub fn inbox(&self) -> ListingRequestBuilder<Inbox> {
        let url = "https://oauth.reddit.com/message/inbox/.json";
        ListingRequestBuilder::new(self.client, url)
    }

    /// Get users unread messages
    pub fn unread(&self) -> ListingRequestBuilder<Inbox> {
        let url = "https://oauth.reddit.com/message/unread/.json";
        ListingRequestBuilder::new(self.client, url)
    }

    /// Get users sent messages
    pub fn sent(&self) -> ListingRequestBuilder<Inbox> {
        let url = "https://oauth.reddit.com/message/sent/.json";
        ListingRequestBuilder::new(self.client, url)
    }

    /// Get submitted
    pub fn submitted(&self) -> ListingRequestBuilder<Submissions> {
        let url = format!(
            "https://oauth.reddit.com/user/{}/submitted/.json",
            self.client.config.username.to_owned().unwrap()
        );
        ListingRequestBuilder::new(self.client, &url)
    }

    /// Get hidden
    pub fn hidden(&self) -> ListingRequestBuilder<Submissions> {
        let url = format!(
            "https://oauth.reddit.com/user/{}/hidden/.json",
            self.client.config.username.to_owned().unwrap()
        );
        ListingRequestBuilder::new(self.client, &url)
    }

    /// Get saved
    pub fn saved(&self) -> ListingRequestBuilder<Submissions> {
        let url = format!(
            "https://oauth.reddit.com/user/{}/saved/.json",
            self.client.config.username.to_owned().unwrap()
        );
        ListingRequestBuilder::new(self.client, &url)
    }

    /// Get upvoted
    pub fn upvoted(&self) -> ListingRequestBuilder<Submissions> {
        let url = format!(
            "https://oauth.reddit.com/user/{}/upvoted/.json",
            self.client.config.username.to_owned().unwrap()
        );
        ListingRequestBuilder::new(self.client, &url)
    }

    /// Get downvoted
    pub fn downvoted(&self) -> ListingRequestBuilder<Submissions> {
        let url = format!(
            "https://oauth.reddit.com/user/{}/downvoted/.json",
            self.client.config.username.to_owned().unwrap()
        );
        ListingRequestBuilder::new(self.client, &url)
    }

    /// Get gilded
    pub fn gilded(&self) -> ListingRequestBuilder<Submissions> {
        let url = format!(
            "https://oauth.reddit.com/user/{}/gilded/.json",
            self.client.config.username.to_owned().unwrap()
        );
        ListingRequestBuilder::new(self.client, &url)
    }

    /// Get comments
    pub fn comments(&self) -> ListingRequestBuilder<Submissions> {
        let url = format!(
            "https://oauth.reddit.com/user/{}/comments/.json",
            self.client.config.username.to_owned().unwrap()
        );
        ListingRequestBuilder::new(self.client, &url)
    }

    // TODO change ListingRequestBuilder into custom
    // TODO comments
}

#[cfg(test)]
mod tests {
    use crate::client::Config;

    use super::Reddit;
    use tokio;

    async fn get_reddit() -> Reddit {
        let user_agent = "reddit api wrapper v1.0 by /u/rickhuis";
        let client_id = "anWiP5x4S6dQJw";
        let client_secret = "rCCer2PLP4CYSKpPy0P-tm7iA6TcrQ";

        let config = Config::new(user_agent, client_id, client_secret)
            .username("testaccountfoobar")
            .password("testaccountfoobar")
            .login()
            .await
            .unwrap();

        Reddit::new(config)
    }

    #[tokio::test]
    async fn inbox() {
        let reddit = get_reddit().await;
        let me = reddit.me();

        let inbox = me.inbox().send().await;
        println!("{:?}", inbox.unwrap());
    }

    #[tokio::test]
    async fn unread() {
        let reddit = get_reddit().await;
        let me = reddit.me();

        let unread = me.unread().send().await;
        println!("{:?}", unread.unwrap());
    }

    #[tokio::test]
    async fn sent() {
        let reddit = get_reddit().await;
        let me = reddit.me();

        let sent = me.sent().send().await;
        println!("{:?}", sent.unwrap());
    }

    #[tokio::test]
    async fn submitted() {
        let reddit = get_reddit().await;
        let me = reddit.me();

        let hidden = me.submitted().send().await;

        assert!(hidden.is_ok());
    }

    #[tokio::test]
    async fn hidden() {
        let reddit = get_reddit().await;
        let me = reddit.me();

        let hidden = me.hidden().send().await;

        assert!(hidden.is_ok());
    }

    #[tokio::test]
    async fn saved() {
        let reddit = get_reddit().await;
        let me = reddit.me();

        let saved = me.saved().send().await;

        assert!(saved.is_ok());
    }

    #[tokio::test]
    async fn upvoted() {
        let reddit = get_reddit().await;
        let me = reddit.me();

        let upvoted = me.upvoted().send().await;

        assert!(upvoted.is_ok());
    }

    #[tokio::test]
    async fn downvoted() {
        let reddit = get_reddit().await;
        let me = reddit.me();

        let downvoted = me.downvoted().send().await;

        assert!(downvoted.is_ok());
    }

    #[tokio::test]
    async fn gilded() {
        let reddit = get_reddit().await;
        let me = reddit.me();

        let gilded = me.gilded().send().await;

        assert!(gilded.is_ok());
    }

    #[tokio::test]
    async fn comments() {
        let reddit = get_reddit().await;
        let me = reddit.me();

        let comments = me.comments().send().await;

        assert!(comments.is_ok());
    }
}
