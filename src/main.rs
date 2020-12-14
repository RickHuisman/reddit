use client::{Config, Reddit};

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

pub mod client;

#[tokio::main]
async fn main() {
    let user_agent = "reddit api wrapper v1.0 by /u/rickhuis";
    let client_id = "VygjvmTaJ88XqQ";
    let client_secret = "IRxsyHEpufmYIEnMyWEI8TmNINw";

    let config = Config::new(user_agent, client_id, client_secret)
        .username("rickhuis")
        .password("Trap71rick")
        .login()
        .await
        .unwrap();

    let reddit = Reddit::new(config);
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
