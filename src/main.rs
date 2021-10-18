mod app;
mod commands;
use app::create_app;
use commands::handle_match;

// use watchexec::config::ConfigBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = create_app().get_matches();

    handle_match(matches).await.unwrap();

    Ok(())
}
