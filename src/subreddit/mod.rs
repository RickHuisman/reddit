use crate::{
    util::{FeedOption, RedditError, TimeFilter},
    Reddit,
};

pub mod responses;
use responses::{Moderators, Submissions};

use self::responses::Rules;

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

        if let Some(option) = options {
            if let Some(after) = option.after {
                url.push_str(&mut format!("&after={}", after.to_owned()));
            } else if let Some(before) = option.before {
                url.push_str(&mut format!("&before={}", before.to_owned()));
            }

            if let Some(count) = option.count {
                url.push_str(&mut format!("&count={}", count));
            }

            if let Some(limit) = option.limit {
                url.push_str(&mut format!("&limit={}", limit));
            }
        }

        Ok(self
            .client
            .get(&url)
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

    pub async fn random(&self) -> Result<Submissions, RedditError> {
        let url = format!("{}/random.json", self.sub_url);
        Ok(self
            .client
            .get(&url)
            .send()
            .await?
            .json::<Submissions>()
            .await?)
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

    // pub async fn rules(&self) -> Result<Rules, RedditError> {
    //     let url = format!("{}/about/rules/.json", self.sub_url);
    //     Ok(self.client.get(&url).send().await?.json::<Rules>().await?)
    // }
}
