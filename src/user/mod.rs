use crate::client::Reddit;
use crate::util::RedditError;
use reqwest::header::USER_AGENT;

pub mod responses;
use self::responses::about::About;
use self::responses::comments::UserComments;
use self::responses::overview::Overview;
use self::responses::submitted::Submitted;

pub struct User<'a> {
    client: &'a Reddit,
    user: String,
}

impl<'a> User<'a> {
    pub fn new(client: &'a Reddit, user: &str) -> User<'a> {
        User {
            client,
            user: user.to_string(),
        }
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

    pub async fn upvoted(&self) {
        // TODO
        println!("test");
        //let url = format!("https://oath.reddit.com/user/{}/upvoted/.json", self.user);
        let url = "https://api.reddit.com/user/testaccountfoobar/upvoted/.json";
        let user_agent = "reddit api wrapper v1.0 by /u/rickhuis";

        let result = self
            .client
            .client
            .get(url)
            .header(USER_AGENT, user_agent)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("{}", result);
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
    use super::Reddit;
    use tokio;
    use crate::client::Config;

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

        assert!(1 == 1); // TODO
    }

    #[tokio::test]
    async fn submitted() {
        let reddit = get_reddit().await;
        let user = reddit.user("spez");

        let submitted = user.submitted().await;

        assert!(submitted.is_ok());
    }

    #[tokio::test]
    async fn upvoted() {
        let reddit = get_reddit().await;
        let user = reddit.user("testaccountfoobar");

        let upvoted = user.upvoted().await;

        // assert!(upvoted.is_ok());
    }

    #[tokio::test]
    async fn comments() {
        let reddit = get_reddit().await;
        let user = reddit.user("spez");

        let comments = user.comments().await;

        assert!(comments.is_ok());
    }
}
