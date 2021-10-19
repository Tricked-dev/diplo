use diplo::{app::create_app, commands::handle_match};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = create_app().get_matches();

    handle_match(matches).await.unwrap();

    Ok(())
}
