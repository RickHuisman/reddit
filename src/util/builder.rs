use std::{collections::HashMap, marker::PhantomData};

use serde::Deserialize;

use crate::{client::Reddit, RedditError};
use crate::util::ListingOptions;

pub struct ListingRequestBuilder<'a, T: for<'de> Deserialize<'de>> {
    client: &'a Reddit,
    phantom: PhantomData<T>,
    endpoint: String,
    listing_options: ListingOptions,
}

impl<'a, T: for<'de> Deserialize<'de>> ListingRequestBuilder<'a, T> {
    pub fn new(client: &'a Reddit, endpoint: &str) -> ListingRequestBuilder<'a, T> {
        ListingRequestBuilder {
            client,
            endpoint: endpoint.to_string(),
            listing_options: ListingOptions::new(),
            phantom: PhantomData, // TODO
        }
    }

    // TODO: send or sent?
    pub async fn send(self) -> Result<T, RedditError> {
        Ok(self.client
            .get(&self.endpoint)
            .query(&self.listing_options.to_params())
            .send()
            .await?
            .json::<T>()
            .await?)
    }

    /// Set after param.
    pub fn after(mut self, after: &str) -> ListingRequestBuilder<'a, T> {
        if self.listing_options.before.is_some() {
            // TODO: Return error.
            panic!("Cannot have an after and before param at the same time");
        }

        self.listing_options.after = Some(after.to_string());
        self
    }

    /// Set before param.
    pub fn before(mut self, before: &str) -> ListingRequestBuilder<'a, T> {
        if self.listing_options.after.is_some() {
            // TODO: Return error.
            panic!("Cannot have an after and before param at the same time");
        }

        self.listing_options.before = Some(before.to_string());
        self
    }

    /// Set count param.
    pub fn count(mut self, count: u32) -> ListingRequestBuilder<'a, T> {
        self.listing_options.count = Some(count);
        self
    }

    /// Set maximum number of items (default: 25, maximum: 100).
    pub fn limit(mut self, limit: u32) -> ListingRequestBuilder<'a, T> {
        self.listing_options.limit = Some(limit);
        self
    }
}

pub struct SubredditRequestBuilder<'a, T: for<'de> Deserialize<'de>> {
    client: &'a Reddit,
    phantom: PhantomData<T>,
    endpoint: String,
    listing_options: ListingOptions,
}

impl<'a, T: for<'de> Deserialize<'de>> SubredditRequestBuilder<'a, T> {
    pub fn new(client: &'a Reddit, endpoint: &str) -> SubredditRequestBuilder<'a, T> {
        Self {
            client,
            endpoint: endpoint.to_string(),
            listing_options: ListingOptions::new(),
            phantom: PhantomData, // TODO
        }
    }

    // TODO: Maybe turn into a trait?
    pub async fn send(self) -> Result<T, RedditError> {
        Ok(self.client
            .get(&self.endpoint)
            .query(&self.listing_options.to_params())
            .send()
            .await?
            .json::<T>()
            .await?)
    }

    /// Set after param.
    pub fn after(mut self, after: &str) -> SubredditRequestBuilder<'a, T> {
        if self.listing_options.before.is_some() {
            // TODO: Return error.
            panic!("Cannot have an after and before param at the same time");
        }

        self.listing_options.after = Some(after.to_string());
        self
    }

    /// Set before param.
    pub fn before(mut self, before: &str) -> SubredditRequestBuilder<'a, T> {
        if self.listing_options.after.is_some() {
            // TODO: Return error.
            panic!("Cannot have an after and before param at the same time");
        }

        self.listing_options.before = Some(before.to_string());
        self
    }

    /// Set count param.
    pub fn count(mut self, count: u32) -> SubredditRequestBuilder<'a, T> {
        self.listing_options.count = Some(count);
        self
    }

    /// Set maximum number of items (default: 25, maximum: 100).
    pub fn limit(mut self, limit: u32) -> SubredditRequestBuilder<'a, T> {
        self.listing_options.limit = Some(limit);
        self
    }
}