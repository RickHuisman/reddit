use crate::client::Reddit;
use crate::util::RedditError;

pub mod responses;
use self::responses::overview::Overview;

pub struct User<'a> {
    client: &'a Reddit,
}

impl<'a> User<'a> {
    pub fn create_new(client: &'a Reddit, name: &str) -> User<'a> {
        User { client }
    }

    pub async fn overview(&self) -> Result<Overview, RedditError> {
        let url = &format!("https://www.reddit.com/user/{}/overview/.json", "rickhuis");
        Ok(self
            .client
            .get(url)
            .send()
            .await?
            .json::<Overview>()
            .await?)
    }
    // pub async fn about(&self) -> Result<UserAboutData, RedditError> {
    //     // let url = format!("https://oauth.reddit.com/user/rickhuis/about?raw_json=1");
    //     let url = format!("https://api.reddit.com/user/rickhuis/about.json");
    //     Ok(self
    //         .client
    //         .get(&url)
    //         .send()
    //         .await?
    //         .json::<UserAboutData>()
    //         .await?)
    // }
}

// #[cfg(test)]
// mod tests {
//     use crate::client::{Config, Reddit};

//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }

//     #[tokio::test]
//     async fn test() {
//         let user_agent = "reddit api wrapper v1.0 by /u/rickhuis";
//         let client_id = "VygjvmTaJ88XqQ";
//         let client_secret = "IRxsyHEpufmYIEnMyWEI8TmNINw";
//         let config = Config::new(user_agent, client_id, client_secret)
//             .username("rickhuis")
//             .password("Trap71rick")
//             .login()
//             .await
//             .unwrap();

//         let reddit = Reddit::new(config);
//         let overview = reddit.user("rickhuis").overview().await;

//         assert!()
//     }
// }
