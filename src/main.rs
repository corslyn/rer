mod api;

use color_eyre::Result;
use dotenv::dotenv;

fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv().ok();
    let api_key = std::env::var("API_KEY")?;
    println!("api_key: {}", api_key);
    let gare = "462941"; // Chatelet les Halles
    let response = api::request(gare, &api_key)?;
    println!("response: {}", response);
    Ok(())
}
