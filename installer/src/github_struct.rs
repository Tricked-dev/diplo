/// Generated by https://quicktype.io
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GithubResponse {
    #[serde(rename = "assets")]
    pub assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "name")]
    pub name: String,

    pub browser_download_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    #[serde(rename = "login")]
    login: String,

    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "node_id")]
    node_id: String,

    #[serde(rename = "avatar_url")]
    avatar_url: String,

    #[serde(rename = "gravatar_id")]
    gravatar_id: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "html_url")]
    html_url: String,

    #[serde(rename = "followers_url")]
    followers_url: String,

    #[serde(rename = "following_url")]
    following_url: String,

    #[serde(rename = "gists_url")]
    gists_url: String,

    #[serde(rename = "starred_url")]
    starred_url: String,

    #[serde(rename = "subscriptions_url")]
    subscriptions_url: String,

    #[serde(rename = "organizations_url")]
    organizations_url: String,

    #[serde(rename = "repos_url")]
    repos_url: String,

    #[serde(rename = "events_url")]
    events_url: String,

    #[serde(rename = "received_events_url")]
    received_events_url: String,

    #[serde(rename = "type")]
    author_type: String,

    #[serde(rename = "site_admin")]
    site_admin: bool,
}