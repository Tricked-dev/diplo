use colored::Colorize;
pub fn load_env(data: Option<bool>) {
    if let Some(data) = data {
        if data {
            if dotenv::dotenv().is_err() {
                println!(
                    "{}",
                    format!("no .env file found continuing without loading dotenv").dimmed(),
                );
            }
        }
    }
}
