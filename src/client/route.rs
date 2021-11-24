use std::fmt;

pub enum Route {
    SubredditHot(String),
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base = "https://oauth.reddit.com";
        let route = match self {
            Route::SubredditHot(sub) => format!("/r/{}/hot", sub),
        };
        write!(f, "{}{}", base, route)
    }
}