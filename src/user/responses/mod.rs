//! # Subreddit Responses
pub mod user;

pub mod about;
pub use about::AboutData;

pub mod overview;
pub use overview::OverviewData;

pub mod submitted;
pub use submitted::SubmittedData;

pub mod comments;
pub use comments::UserCommentsData;
