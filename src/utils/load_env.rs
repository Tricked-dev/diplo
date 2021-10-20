use colored::Colorize;
pub fn load_env(data: Option<bool>) {
    if data.is_some() && dotenv::dotenv().is_err() {
        println!(
            "{} {}",
            ">".red().to_string(),
            "no .env file found continuing without loading dotenv"
                .dimmed()
                .to_string(),
        );
    }
}
