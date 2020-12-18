use client::{Config, Reddit};
use std::env;

pub mod responses;

/// Me module.
pub mod me;
pub use me::Me;

/// Subreddit module.
pub mod subreddit;
pub use subreddit::Subreddit;

/// User module.
pub mod user;
pub use user::User;

/// Utils for requests.
pub mod util;
pub use util::RedditError;

pub mod client;

async fn get_config() -> Result<Config, RedditError> {
    let user_agent = "reddit api wrapper v1.0 by /u/rickhuis";
    let client_id = "anWiP5x4S6dQJw";
    let client_secret = "rCCer2PLP4CYSKpPy0P-tm7iA6TcrQ";

    Ok(Config::new(user_agent, client_id, client_secret)
        .username("testaccountfoobar")
        .password("testaccountfoobar")
        .login()
        .await?)
}

#[tokio::main]
async fn main() {
    /* let reddit = Reddit::new(config);
     * reddit.subreddit.
     * 
     */

    // let config = get_config().await.unwrap();

    // let reddit = Reddit::new(config);
    // let overview = reddit.user("rickhuis").overview();

    // reddit.subreddit("soccer").search().await;
    // let feed = reddit.subreddit("formula1").hot(None).await.unwrap();
    // for s in feed.data.children {
    //     println!("{}", s.data.title);
    // }

    // reddit.subreddit("soccer").subscribe().await;
    // let moderators = reddit.subreddit("soccer").moderators().await;
    // if let Ok(mods) = moderators {
    //     for m in mods.data.children {
    //         println!("{}", m.name);
    //     }
    // }

    // reddit.user("rickhuis").overview().await.unwrap();
    // let feed = reddit.me().saved().await;
    // for s in feed.unwrap().data.children {
    //     println!("Title: {}", s.data.title);
    // }

    // if let Ok(medata) = reddit.me().me().await {
    //     println!("{}", medata.created_utc);
    // }
    // println!("{:?}", reddit.me().test());

    // let option = FeedOption::new().limit(5);
    // let feed = reddit
    //     .subreddit("soccer")
    //     .top(Some(option), util::TimeFilter::Month)
    //     .await;

    // for s in feed.unwrap().data.children {
    //     println!("Title: {}", s.data.title);
    // }
}
