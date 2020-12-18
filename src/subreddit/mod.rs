use crate::{
    util::{FeedOption, RedditError, TimeFilter},
    Reddit,
    Config,
};

use std::collections::HashMap;
use reqwest::Response;

pub mod responses;
use responses::{Moderators, Submissions};

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
            sub_url: format!("https://oauth.reddit.com/r/{}", name.to_owned()),
            client,
        }
    }

    async fn get_feed(
        &self,
        endpoint: &str,
        options: Option<FeedOption>,
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
                params.insert("count", count.to_owned().to_string());
            }

            if let Some(limit) = option.limit {
                params.insert("limit", limit.to_owned().to_string());
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

    pub async fn new(&self, option: Option<FeedOption>) -> Result<Submissions, RedditError> {
        self.get_feed("new", option).await
    }

    pub async fn hot(&self, option: Option<FeedOption>) -> Result<Submissions, RedditError> {
        self.get_feed("hot", option).await
    }

    pub async fn rising(&self, option: Option<FeedOption>) -> Result<Submissions, RedditError> {
        self.get_feed("rising", option).await
    }

    pub async fn top(
        &self,
        option: Option<FeedOption>,
        time: TimeFilter,
    ) -> Result<Submissions, RedditError> {
        let path = format!("top?{}&", time);
        self.get_feed(&path, option).await
    }

    pub async fn controversial(
        &self,
        option: Option<FeedOption>,
        time: TimeFilter,
    ) -> Result<Submissions, RedditError> {
        let path = format!("controversial?{}&", time);
        self.get_feed(&path, option).await
    }

    pub async fn moderators(&self) -> Result<Moderators, RedditError> {
        let url = format!("{}/about/moderators/.json", self.sub_url);
        Ok(self
            .client
            .get(&url)
            .send()
            .await?
            .json::<Moderators>()
            .await?)
    }

    pub async fn search(&self) {
        // TODO
        let url = format!("{}/search/.json", self.sub_url);

        let result = self
            .client
            .get(&url)
            .send()
            .await
            .unwrap();

        println!("{:?}", url);
        println!("{:?}", result);
    }

    fn build_oath_url(&self, dest: &str) -> String {
        format!("https://oauth.reddit.com/{}", dest)
    }

    pub async fn subscribe(&self) -> Result<Response, RedditError> {
        let url = self.build_oath_url("/api/subscribe/");
        let params = [("action", "sub"), ("sr_name", &self.name)];

        Ok(self
            .client
            .post(&url)
            .form(&params)
            .send()
            .await?)
    }

    pub async fn unsubscribe(&self) -> Result<Response, RedditError> {
        let url = self.build_oath_url("/api/subscribe/");
        let params = [("action", "unsub"), ("sr_name", &self.name)];

        Ok(self
            .client
            .post(&url)
            .form(&params)
            .send()
            .await?)
    }

    // pub async fn rules(&self) -> Result<Rules, RedditError> {
    //     let url = format!("{}/about/rules/.json", self.sub_url);
    //     Ok(self.client.get(&url).send().await?.json::<Rules>().await?)
    // }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use super::Reddit;
    use super::TimeFilter;
    use super::Subreddit;
    use super::FeedOption;
    use tokio;

    async fn get_reddit() -> Reddit {
        // TODO Get from env
        let user_agent = "reddit api wrapper v1.0 by /u/rickhuis";
        let client_id = "VygjvmTaJ88XqQ";
        let client_secret = "IRxsyHEpufmYIEnMyWEI8TmNINw";

        let config = Config::new(user_agent, client_id, client_secret)
            .username("rickhuis")
            .password("Trap71rick")
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
        let new = subreddit.new(None).await;
        assert!(new.is_ok());

        let hot = subreddit.hot(None).await;
        assert!(hot.is_ok());

        let option = Some(FeedOption::new().limit(5));
        let rising = subreddit.rising(option).await;
        assert!(rising.is_ok());

        let rising_len = rising.unwrap().data.children.len();
        assert!(rising_len == 5);

        let top = subreddit.top(None, TimeFilter::Month).await;
        assert!(top.is_ok());

        let controversial = subreddit.controversial(None, TimeFilter::Month).await;
        assert!(controversial.is_ok());

        // Test moderators
        let moderators = subreddit.moderators().await;
        assert!(moderators.is_ok());
    }
}
