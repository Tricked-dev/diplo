use crate::{info, term::print_inner};
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct GithubRelease {
    #[serde(rename = "tag_name")]
    tag_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Versions {
    pub latest: String,
}

lazy_static! {
    pub static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::builder()
        .gzip(true)
        .brotli(true)
        .build()
        .unwrap();
    pub static ref PATH: Regex = Regex::new("/(.*).(ts|js)").unwrap();
    pub static ref VERSION: Regex = Regex::new("@(.*)").unwrap();
}

pub async fn get_latest_x_module(name: &str) -> String {
    let res = HTTP_CLIENT
        .get(format!("https://cdn.deno.land/{}/meta/versions.json", name))
        .header("user-agent", "diplo")
        .send()
        .await
        .unwrap();
    let text = res.text().await.unwrap();

    let json: Versions = serde_json::from_str(&text).unwrap();
    json.latest
}

pub async fn get_latest_std() -> String {
    let res = HTTP_CLIENT
        .get("https://api.github.com/repos/denoland/deno_std/releases/latest")
        .header("user-agent", "diplo")
        .send()
        .await
        .unwrap();
    let text = res.text().await.unwrap();

    let json: GithubRelease = serde_json::from_str(&text).unwrap();
    json.tag_name
}

pub async fn update_deno_std(val: String) -> Result<String> {
    let part = val.replace("https://deno.land/std", "");
    let part2 = PATH.captures(&part).unwrap().get(0).unwrap();
    let part3 = PATH.replace(&part, "");
    let ver = VERSION.captures(&part3);

    let version: &str;
    if let Some(ver) = ver {
        version = ver.get(1).unwrap().as_str()
    } else {
        version = "0"
    }
    let latest_std = get_latest_std().await;
    if version != latest_std {
        info!("updated std to {} from {}", latest_std, version);
        Ok(format!(
            "https://deno.land/std@{}{}",
            latest_std,
            part2.as_str()
        ))
    } else {
        Err(anyhow!(""))
    }
}
pub async fn update_deno_x(val: String) -> Result<String> {
    let part = val.replace("https://deno.land/x/", "");
    let part2 = PATH.captures(&part).unwrap().get(0).unwrap();
    let part3 = PATH.replace(&part, "");
    let ver = VERSION.captures(&part3);
    let name = VERSION.replace(&part3, "");

    let version: &str;
    if let Some(ver) = ver {
        version = ver.get(1).unwrap().as_str()
    } else {
        version = "0"
    }

    let new_version = get_latest_x_module(&name).await;
    if version != new_version {
        info!("updated {} to {} from {}", name, new_version, version);
        Ok(format!(
            "https://deno.land/x/{}@{}{}",
            name,
            new_version,
            part2.as_str()
        ))
    } else {
        Err(anyhow!(""))
    }
}

pub async fn update_deps(deps: &HashMap<String, String>) -> HashMap<String, String> {
    let mut data: HashMap<String, String> = HashMap::new();
    for (key, val) in deps.iter() {
        data.insert((&key).to_string(), (&val).to_string());
        //https://cdn.deno.land/natico/meta/versions.json
        //https://cdn.deno.land/natico/versions/3.0.0-rc.1/meta/meta.json
        //https://deno.land/x/natico@3.0.0-rc.1/doc_mod.ts
        if val.contains("https://deno.land/x/") {
            if let Ok(result) = update_deno_x(val.to_string()).await {
                data.insert((&key).to_string(), result);
            }
        }
        if val.contains("https://deno.land/std") {
            if let Ok(result) = update_deno_std(val.to_string()).await {
                data.insert((&key).to_string(), result);
            }
        }
    }
    data
}

#[cfg(test)]
mod tests {
    use super::update_deps;
    use std::collections::HashMap;

    #[cfg(not(target_os = "macos"))]
    #[tokio::test]
    async fn update_some_deps() {
        let mut deps: HashMap<String, String> = HashMap::new();

        deps.insert(
            "natico".to_owned(),
            "https://deno.land/x/natico@2.3.0-rc.2/mod.ts".to_owned(),
        );
        deps.insert(
            "discordeno".to_owned(),
            "https://deno.land/x/natico@2.3.0-rc.2/mod.ts".to_owned(),
        );
        deps.insert(
            "lodash".to_owned(),
            "https://deno.land/x/lodash@4.17.19/dist/lodash.core.js".to_owned(),
        );
        deps.insert(
            "crypto".to_owned(),
            "https://deno.land/std@0.111.0/node/crypto.ts".to_owned(),
        );

        update_deps(&deps).await;
    }
}
