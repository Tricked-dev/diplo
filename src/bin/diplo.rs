use diplo::{app::create_app, commands::handle_match};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = create_app().get_matches();

    handle_match(matches).await.unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{read_to_string, write};

    #[tokio::test]
    async fn run_init() {
        let matches = create_app().get_matches_from(vec!["diplo", "init", "-y"]);
        handle_match(matches).await.unwrap()
    }

    #[cfg(not(target_os = "macos"))]
    #[tokio::test]
    async fn run_add() {
        handle_match(create_app().get_matches_from(vec!["diplo", "add", "natico", "discordeno"]))
            .await
            .unwrap();
        handle_match(create_app().get_matches_from(vec!["diplo", "add", "--std", "fs", "ws"]))
            .await
            .unwrap();
    }
    //For some reason macos cant reach github api?
    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn run_add() {
        handle_match(create_app().get_matches_from(vec!["diplo", "add", "natico", "discordeno"]))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn run_cache() {
        handle_match(create_app().get_matches_from(vec!["diplo", "cache"]))
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn run_exec() {
        handle_match(create_app().get_matches_from(vec!["diplo", "exec", "ls"]))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn run_update() {
        handle_match(create_app().get_matches_from(vec!["diplo", "update"]))
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn run_install() {
        handle_match(create_app().get_matches_from(vec!["diplo", "install"]))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn run_script() {
        let mut data = read_to_string("diplo.toml").unwrap();
        data.push_str("\ntest=\"ls -la\"");
        write("diplo.toml", data).unwrap();
        handle_match(create_app().get_matches_from(vec!["diplo", "run", "test"]))
            .await
            .unwrap();
    }

    //It somehow cant reach github api-servers on macos
    #[cfg(not(target_os = "macos"))]
    #[tokio::test]
    async fn update_some_deps() {
        // let mut deps: HashMap<String, String> = HashMap::new();

        // deps.insert(
        //     "natico".to_owned(),
        //     "https://deno.land/x/natico@2.3.0-rc.2/mod.ts".to_owned(),
        // );
        // deps.insert(
        //     "discordeno".to_owned(),
        //     "https://deno.land/x/natico@2.3.0-rc.2/mod.ts".to_owned(),
        // );
        // deps.insert(
        //     "lodash".to_owned(),
        //     "https://deno.land/x/lodash@4.17.19/dist/lodash.core.js".to_owned(),
        // );
        // deps.insert(
        //     "crypto".to_owned(),
        //     "https://deno.land/std@0.111.0/node/crypto.ts".to_owned(),
        // );

        // update_deps(&deps).await;
    }
}
