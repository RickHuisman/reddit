use crate::{
    util::{
        builder::SubredditRequestBuilder, ListingOptions, RedditError,
        TimeFilter,
    },
    Reddit,
};

use reqwest::Response;

use std::collections::HashMap;

pub mod responses;
use responses::{Moderators, Submissions};
use crate::client::route::Route;

pub struct Subreddit<'a> {
    /// Name of subreddit.
    pub name: String,
    /// Url of subreddit.
    sub_url: String,
    client: &'a Reddit,
}

impl<'a> Subreddit<'a> {
    pub fn create_new(client: &'a Reddit, name: &str) -> Subreddit<'a> {
        Subreddit {
            name: name.to_owned(),
            sub_url: format!("https://oauth.reddit.com/r/{}", name.to_string()), // TODO: Ugly.
            client,
        }
    }

    async fn get_feed(
        &self,
        endpoint: &str,
        options: Option<ListingOptions>,
    ) -> Result<Submissions, RedditError> {
        let url = &mut format!("{}/{}.json?", self.sub_url, endpoint);
        let mut params = HashMap::new();

        if let Some(option) = options {
            if let Some(after) = option.after {
                params.insert("after", after.to_owned());
            } else if let Some(before) = option.before {
                params.insert("before", before.to_owned());
            }

            if let Some(count) = option.count {
                params.insert("count", count.to_string());
            }

            if let Some(limit) = option.limit {
                params.insert("limit", limit.to_string());
            }
        }

        Ok(self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json::<Submissions>()
            .await?)
    }

    pub fn new(&self) -> SubredditRequestBuilder<Submissions> {
        let url = format!(
            "https://oauth.reddit.com/r/{}/new/.json?",
            self.name.to_owned()
        );
        SubredditRequestBuilder::new(self.client, &url)
    }

    pub fn hot(&self) -> SubredditRequestBuilder<Submissions> {
        // let url = Route::SubredditHot(self.sub_url.to_string()); TODO
        let url = format!(
            "https://oauth.reddit.com/r/{}/hot/.json?",
            self.name.to_owned()
        );
        SubredditRequestBuilder::new(self.client, &url)
    }

    pub fn rising(&self) -> SubredditRequestBuilder<Submissions> {
        let url = format!(
            "https://oauth.reddit.com/r/{}/rising/.json?",
            self.name.to_owned()
        );
        SubredditRequestBuilder::new(self.client, &url)
    }

    pub fn top(&self, time: TimeFilter) -> SubredditRequestBuilder<Submissions> {
        let path = format!("top?{}&", time);
        let url = &mut format!("{}/{}.json?", self.sub_url, path);

        SubredditRequestBuilder::new(self.client, &url)
    }

    pub fn controversial(&self, time: TimeFilter) -> SubredditRequestBuilder<Submissions> {
        let path = format!("controversial?{}&", time);
        let url = &mut format!("{}/{}.json?", self.sub_url, path);

        SubredditRequestBuilder::new(self.client, &url)
    }

    // TODO
    // pub fn moderators(&self) -> ModeratorRequestBuilder<Moderators> {
    //     let url = format!(
    //         "https://oauth.reddit.com/r/{}/about/moderators/.json?",
    //         self.name.to_owned()
    //     );
    //     ModeratorRequestBuilder::new(&url)
    // }

    pub async fn search(&self) {
        // TODO
        let url = format!("{}/search/.json", self.sub_url);

        let result = self.client.get(&url).send().await.unwrap();

        println!("{:?}", url);
        println!("{:?}", result);
    }

    // TODO: Move.
    fn build_oath_url(&self, dest: &str) -> String {
        format!("https://oauth.reddit.com/{}", dest)
    }

    pub async fn subscribe(&self) -> Result<Response, RedditError> {
        let url = self.build_oath_url("/api/subscribe/");
        let params = [("action", "sub"), ("sr_name", &self.name)];

        Ok(self.client.post(&url).form(&params).send().await?)
    }

    pub async fn unsubscribe(&self) -> Result<Response, RedditError> {
        let url = self.build_oath_url("/api/subscribe/");
        let params = [("action", "unsub"), ("sr_name", &self.name)];

        Ok(self.client.post(&url).form(&params).send().await?)
    }

    // TODO
    // pub async fn rules(&self) -> Result<Rules, RedditError> {
    //     let url = format!("{}/about/rules/.json", self.sub_url);
    //     Ok(self.client.get(&url).send().await?.json::<Rules>().await?)
    // }
}

#[cfg(test)]
mod tests {
    use crate::client::Config;

    use super::Reddit;
    use super::TimeFilter;
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
    async fn subreddits() {
        let reddit = get_reddit().await;
        let subreddit = reddit.subreddit("soccer");

        // Test feeds
        // let new = subreddit.new().send(reddit).await;
        // assert!(new.is_ok());

        // let hot = subreddit.hot().send(reddit).await;
        // assert!(hot.is_ok());

        // let rising = subreddit.rising().send(reddit).await;
        // assert!(rising.is_ok());

        // let rising_len = rising.unwrap().data.children.len();
        // assert!(rising_len == 5);

        let top = subreddit.top(None, TimeFilter::Month).await;
        assert!(top.is_ok());

        let controversial = subreddit.controversial(None, TimeFilter::Month).await;
        assert!(controversial.is_ok());

        // TODO:
        // Test moderators
        // let moderators = subreddit.moderators().send().await;
        // assert!(moderators.is_ok());
    }

    #[tokio::test]
    async fn subscribe() {
        let reddit = get_reddit().await;
        let subreddit = reddit.subreddit("soccer");

        let result = subreddit.subscribe().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn unsubscribe() {
        let reddit = get_reddit().await;
        let subreddit = reddit.subreddit("soccer");

        let result = subreddit.unsubscribe().await;

        assert!(result.is_ok());
    }
}
