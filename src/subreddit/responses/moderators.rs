use serde::Deserialize;

use crate::responses::{basic::Listing, BasicThing};

#[derive(Debug, Deserialize)]
pub struct ModeratorsData {
    pub name: String,
    pub author_flair_text: Option<String>,
    pub mod_permissions: Vec<String>,
    pub date: f64,
    pub rel_id: String,
    pub id: String,
    pub author_flair_css_class: Option<String>,
}

pub type Moderators = BasicThing<Listing<ModeratorsData>>;
