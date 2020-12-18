use serde::Deserialize;

use crate::responses::BasicThing;

#[derive(Debug, Deserialize)]
pub struct AboutData {
    pub is_employee: bool,
    pub is_friend: bool,
    pub subreddit: Subreddit,
    pub snoovatar_size: ::serde_json::Value,
    pub awardee_karma: i64,
    pub id: String,
    pub verified: bool,
    pub is_gold: bool,
    pub is_mod: bool,
    pub awarder_karma: i64,
    pub has_verified_email: bool,
    pub icon_img: String,
    pub hide_from_robots: bool,
    pub link_karma: i64,
    pub total_karma: i64,
    pub pref_show_snoovatar: bool,
    pub name: String,
    pub created: f64,
    pub created_utc: f64,
    pub snoovatar_img: String,
    pub comment_karma: i64,
    pub has_subscribed: bool,
}

// TODO Move to subreddit responses
#[derive(Debug, Deserialize)]
pub struct Subreddit {
    pub default_set: bool,
    pub user_is_contributor: ::serde_json::Value,
    pub banner_img: String,
    pub restrict_posting: bool,
    pub user_is_banned: ::serde_json::Value,
    pub free_form_reports: bool,
    pub community_icon: ::serde_json::Value,
    pub show_media: bool,
    pub icon_color: String,
    pub user_is_muted: ::serde_json::Value,
    pub display_name: String,
    pub header_img: ::serde_json::Value,
    pub title: String,
    pub previous_names: Vec<::serde_json::Value>,
    pub over_18: bool,
    pub icon_size: Vec<i64>,
    pub primary_color: String,
    pub icon_img: String,
    pub description: String,
    pub submit_link_label: String,
    pub header_size: ::serde_json::Value,
    pub restrict_commenting: bool,
    pub subscribers: i64,
    pub submit_text_label: String,
    pub is_default_icon: bool,
    pub link_flair_position: String,
    pub display_name_prefixed: String,
    pub key_color: String,
    pub name: String,
    pub is_default_banner: bool,
    pub url: String,
    pub quarantine: bool,
    pub banner_size: ::serde_json::Value,
    pub user_is_moderator: ::serde_json::Value,
    pub public_description: String,
    pub link_flair_enabled: bool,
    pub disable_contributor_requests: bool,
    pub subreddit_type: String,
    pub user_is_subscriber: ::serde_json::Value,
}

/// About
pub type About = BasicThing<AboutData>;
