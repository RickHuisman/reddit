use crate::client::{Reddit, Config};
use crate::util::RedditError;
use reqwest::header::USER_AGENT;

pub mod responses;
use self::responses::about::About;
use self::responses::overview::Overview;
use self::responses::submitted::Submitted;
use self::responses::comments::UserComments;

pub struct User<'a> {
    client: &'a Reddit,
    user: String,
}

impl<'a> User<'a> {
    pub fn create_new(client: &'a Reddit, user: &str) -> User<'a> {
        User { client, user: user.to_string() }
    }

    pub async fn about(&self) -> Result<About, RedditError> {
         let url = format!("https://api.reddit.com/user/{}/about.json", self.user);
         let user_agent = "reddit api wrapper v1.0 by /u/rickhuis";

         Ok(self
             .client
             .client
             .get(&url)
             .header(USER_AGENT, user_agent)
             .send()
             .await?
             .json::<About>()
             .await?)
    }

    pub async fn overview(&self) -> Result<Overview, RedditError> {
         let url = format!("https://api.reddit.com/user/{}/overview.json", self.user);
         let user_agent = "reddit api wrapper v1.0 by /u/rickhuis";

         Ok(self
             .client
             .client
             .get(&url)
             .header(USER_AGENT, user_agent)
             .send()
             .await?
             .json::<Overview>()
             .await?)
    }

    // pub async fn upvoted(&self) {
    //     let url = format!("https://oath.reddit.com/user/{}/upvoted/.json", self.user);

    //     let result = self
    //         .client
    //         .get(&url)
    //         .send()
    //         .await.unwrap()
    //         .text()
    //         .await.unwrap();

    //     println!("{}", result);
    // }

    pub async fn submitted(&self) -> Result<Submitted, RedditError> {
        let url = format!("https://api.reddit.com/user/{}/submitted/.json", self.user);
        let user_agent = "reddit api wrapper v1.0 by /u/rickhuis";

        Ok(self
            .client
            .client
            .get(&url)
            .header(USER_AGENT, user_agent)
            .send()
            .await?
            .json::<Submitted>()
            .await?)
    }

    pub async fn comments(&self) -> Result<UserComments, RedditError> {
        let url = format!("https://api.reddit.com/user/{}/comments/.json", self.user);
        let user_agent = "reddit api wrapper v1.0 by /u/rickhuis";

        Ok(self
            .client
            .client
            .get(&url)
            .header(USER_AGENT, user_agent)
            .send()
            .await?
            .json::<UserComments>()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use super::Reddit;
    use tokio;

    async fn get_reddit() -> Reddit {
        // TODO Get from env
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
    async fn about() {
        let reddit = get_reddit().await;
        let user = reddit.user("spez");

        let about = user.about().await;

        assert!(about.is_ok());
    }

    #[tokio::test]
    async fn overview() {
        let reddit = get_reddit().await;
        let user = reddit.user("spez");

        let overview = user.overview().await.unwrap();
        println!("{:?}", overview);

        assert!(1 == 1);
    }

//    #[tokio::test]
//    async fn upvoted() {
//        let reddit = get_reddit().await;
//        let user = reddit.user("rickhuis");
//
//        let upvoted = user.upvoted().await;
//
//        // assert!(upvoted.is_ok());
//    }

    #[tokio::test]
    async fn submitted() {
        let reddit = get_reddit().await;
        let user = reddit.user("spez");

        let submitted = user.submitted().await;

        assert!(submitted.is_ok());
    }

    #[tokio::test]
    async fn comments() {
        let reddit = get_reddit().await;
        let user = reddit.user("spez");

        let comments = user.comments().await;

        assert!(comments.is_ok());
    }
}
