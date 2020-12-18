use serde::Deserialize;
use crate::responses::BasicListing;

#[derive(Debug, Deserialize)]
// TODO Missing fields
pub struct SubmittedData {
    /// Subreddit
    pub subreddit: String,
    /// Title
    pub title: String,
    /// Thumbnail
    pub thumbnail: String,
    /// Score
    pub score: i32,
    /// Created
    pub created: f64,
    /// Domain
    pub domain: String,
    /// Is self
    pub is_self: bool,
}

/// Submitted
pub type Submitted = BasicListing<SubmittedData>;
