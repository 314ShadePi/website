/// Generated by https://quicktype.io
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BlogList {
    #[serde(rename = "posts")]
    posts: Vec<Post>,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "filename")]
    filename: String,

    #[serde(rename = "author")]
    author: String,
}
