use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
}

pub async fn update_deps(deps: &HashMap<String, String>) -> HashMap<String, String> {
    let mut data: HashMap<String, String> = HashMap::new();
    let re = Regex::new("/(.*).ts").unwrap();
    let at = Regex::new("@(.*)").unwrap();
    for (key, val) in deps.iter() {
        data.insert((&key).to_string(), (&val).to_string());
        //https://cdn.deno.land/natico/meta/versions.json
        //https://cdn.deno.land/natico/versions/3.0.0-rc.1/meta/meta.json
        //https://deno.land/x/natico@3.0.0-rc.1/doc_mod.ts
        if val.contains("https://deno.land/x/") {
            let part = val.replace("https://deno.land/x/", "");
            let part2 = re.captures(&part).unwrap().get(0).unwrap();
            let part3 = re.replace(&part, "");
            let ver = at.captures(&part3);
            let name = at.replace(&part3, "");
            println!("{:#?}", ver);
            let version: &str;
            if let Some(ver) = ver {
                version = ver.get(1).unwrap().as_str()
            } else {
                version = "0"
            }
            let res = HTTP_CLIENT
                .get(format!("https://cdn.deno.land/{}/meta/versions.json", name))
                .header("user-agent", "diplo")
                .send()
                .await
                .unwrap();
            let text = res.text().await.unwrap();

            let json: Versions = serde_json::from_str(&text).unwrap();
            if version != json.latest {
                println!("updated {} to {} from {}", name, json.latest, version);
                data.insert(
                    (&key).to_string(),
                    format!(
                        "https://deno.land/x/{}@{}{}",
                        name,
                        json.latest,
                        part2.as_str()
                    ),
                );
            }
        }
    }
    data
}
