//! # User Overview Responses
use serde::Deserialize;

use crate::responses::BasicListing;

/// OverviewData
#[derive(Debug, Deserialize)]
pub struct OverviewData {
    /// Author
    pub author: String,
    /// Likes
    pub likes: Option<i32>,
    /// Score
    pub score: i32,
    /// Subreddit
    pub subreddit: String,
    /// Created
    pub created: f64,
    /// Body
    pub body: String,
    /// Link title
    pub link_title: String,
    /// Link url
    pub link_url: String,
}

/// Overview
pub type Overview = BasicListing<OverviewData>;
