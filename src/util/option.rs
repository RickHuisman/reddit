use std::fmt;

/// Basic feed options
pub struct FeedOption {
    /// `after` and `before` indicate the fullname of an item in the listing to use as the anchor point of the slice.
    pub after: Option<String>,
    /// Only one should be specified.
    pub before: Option<String>,
    /// The number of items already seen in this listing.
    pub count: Option<u32>,
    pub limit: Option<u32>,
}

impl FeedOption {
    /// Create a new `FeedOption` instance.
    pub fn new() -> FeedOption {
        FeedOption {
            after: None,
            before: None,
            count: None,
            limit: None,
        }
    }

    /// Set after param.
    pub fn after(mut self, after: &str) -> FeedOption {
        if !self.before.is_none() {
            panic!("Cannot have an after and before param at the same time");
        }

        self.after = Some(after.to_owned());
        self
    }

    /// Set before param.
    pub fn before(mut self, before: &str) -> FeedOption {
        if !self.after.is_none() {
            panic!("Cannot have an after and before param at the same time");
        }

        self.before = Some(before.to_owned());
        self
    }

    /// Set count param.
    pub fn count(mut self, count: u32) -> FeedOption {
        self.count = Some(count);
        self
    }

    pub fn limit(mut self, limit: u32) -> FeedOption {
        self.limit = Some(limit);
        self
    }
}

pub enum TimeFilter {
    Hour,
    Day,
    Week,
    Month,
    Year,
    AllTime,
}

impl fmt::Display for TimeFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            TimeFilter::Hour => "hour",
            TimeFilter::Day => "day",
            TimeFilter::Week => "week",
            TimeFilter::Month => "month",
            TimeFilter::Year => "year",
            TimeFilter::AllTime => "all",
        };
        write!(f, "&t={}", s)
    }
}
