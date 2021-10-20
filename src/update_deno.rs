use anyhow::{anyhow, Result};
use colored::Colorize;
use hyper::{body::Buf, client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
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
    //Static http client since creating one on every request isn't performant
    pub static ref HTTP_CLIENT: Lazy<Client<HttpsConnector<HttpConnector>>> = Lazy::new(||{
        let https = HttpsConnector::new();
     Client::builder().build::<_, hyper::Body>(https)
    });
    pub static ref PATH: Regex = Regex::new("/(.*).(ts|js)").unwrap();
    pub static ref VERSION: Regex = Regex::new("@(.*)").unwrap();
}
///Takes a deno.land/x module and fetches the latest version for it!, only requires a name
pub async fn get_latest_x_module(name: &str) -> String {
    let url = format!("https://cdn.deno.land/{}/meta/versions.json", name)
        .parse::<hyper::Uri>()
        .unwrap();

    let res = HTTP_CLIENT.get(url).await.unwrap();
    let body = hyper::body::aggregate(res).await.unwrap();

    // try to parse as json with serde_json
    let json: Versions = serde_json::from_reader(body.reader()).unwrap();

    json.latest
}

pub async fn get_latest_std() -> String {
    let req = Request::builder()
        .method(Method::GET)
        .uri("https://api.github.com/repos/denoland/deno_std/releases/latest")
        //Hyper doesn't set a user-agent by default, otherwise github blocks the request
        .header("user-agent", "diplo/rust")
        .body(Body::empty())
        .expect("request builder");

    let res = HTTP_CLIENT.request(req).await.unwrap();

    let body = hyper::body::aggregate(res).await.unwrap();

    let json: GithubRelease = serde_json::from_reader(body.reader()).unwrap();

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
        println!(
            "updated std to {} from {}",
            latest_std.bold(),
            version.bold()
        );
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
        println!(
            "updated {} to {} from {}",
            name.bold(),
            new_version.green(),
            version.red()
        );
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

    //It somehow cant reach github api-servers on macos
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
