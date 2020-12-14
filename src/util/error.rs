use std::error;
use std::fmt;

use reqwest;
use serde_json;

/// Error type that occurs when an API request fails for some reason.
#[derive(Debug)]
pub enum RedditError {
    /// Occurs when the API has returned a non-success error code.
    Status(reqwest::Response),
    /// Occurs if the HTTP response from Reddit was corrupt and
    /// reqwest could not parse it.
    Network(reqwest::Error),
    /// Occurs if serde could not Deserialize the response.
    Parse(serde_json::Error),
}

impl From<reqwest::Error> for RedditError {
    fn from(e: reqwest::Error) -> Self {
        RedditError::Network(e)
    }
}

impl From<serde_json::Error> for RedditError {
    fn from(e: serde_json::Error) -> Self {
        RedditError::Parse(e)
    }
}

impl fmt::Display for RedditError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RedditError::Status(ref err) => write!(f, "Status error: {}", err.status()),
            RedditError::Network(ref err) => err.fmt(f),
            RedditError::Parse(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for RedditError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            RedditError::Status(_) => None,
            RedditError::Network(ref err) => Some(err),
            RedditError::Parse(ref err) => Some(err),
        }
    }
}
