// TODO
// use config::Config;
// use reqwest::Client;
// use reqwest::RequestBuilder;

// // Me module.
// pub mod me;
// pub use me::Me;

// /// Subreddit module.
// pub mod subreddit;
// use reqwest::Client;
// pub use subreddit::Subreddit;

// /// Utils for requests.
// pub mod util;

pub mod config;
pub use config::Config;

pub mod client;
pub mod route;

pub use client::Reddit;
