use std::collections::HashMap;
use std::fmt;

/// Basic feed options
pub struct ListingOptions {
    /// `after` and `before` indicate the fullname of an item in the listing to use as the anchor point of the slice.
    pub after: Option<String>,
    /// Only one should be specified.
    pub before: Option<String>,
    /// The number of items already seen in this listing.
    pub count: Option<u32>, // TODO: Use usize?
    // TODO: Doc.
    pub limit: Option<u32>, // TODO: Use usize?
}

impl ListingOptions {
    /// Create a new `FeedOptions` instance.
    pub fn new() -> Self {
        Self {
            after: None,
            before: None,
            count: None,
            limit: None,
        }
    }

    /// Set after param.
    pub fn after(mut self, after: &str) -> Self {
        if !self.before.is_none() {
            panic!("Cannot have an after and before param at the same time");
        }

        self.after = Some(after.to_string());
        self
    }

    /// Set before param.
    pub fn before(mut self, before: &str) -> Self {
        if !self.after.is_none() {
            panic!("Cannot have an after and before param at the same time");
        }

        self.before = Some(before.to_string());
        self
    }

    /// Set count param.
    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    // TODO: Check lifetime annotation.
    // TODO: Convert to .into()
    pub fn to_params<'a>(self) -> HashMap<&'a str, String> {
        let mut params = HashMap::new();

        if let Some(after) = self.after {
            params.insert("after", after.to_string());
        } else if let Some(before) = self.before {
            params.insert("before", before.to_string());
        }

        if let Some(count) = self.count {
            params.insert("count", count.to_string());
        }
        if let Some(limit) = self.limit {
            params.insert("limit", limit.to_string());
        }

        // TODO
        // if let Some(sr_detail) = self.sr_detail {
        //     params.insert("sr_detail", sr_detail.to_string());
        // }

        params
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
        let filter = match *self {
            TimeFilter::Hour => "hour",
            TimeFilter::Day => "day",
            TimeFilter::Week => "week",
            TimeFilter::Month => "month",
            TimeFilter::Year => "year",
            TimeFilter::AllTime => "all",
        };
        write!(f, "&t={}", filter)
    }
}

// TODO Rename
pub struct UserOptions {
    // TODO
    /// an integer between 2 and 10
    context: u8,
    time: TimeFilter,
    sort: SortOption,
    feed: ListingOptions, // TODO: Rename field.
    include_categories: bool,
    // show: TODO
}

pub enum SortOption {
    Hot,
    New,
    Top,
    Controversial
}

impl fmt::Display for SortOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let option = match *self {
            SortOption::Hot => "hot",
            SortOption::New => "new",
            SortOption::Top => "Top", // TODO: "Top" or "top"?
            SortOption::Controversial => "controversial",
        };
        write!(f, "&s={}", option)
    }
}
