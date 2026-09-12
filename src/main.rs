pub mod api;
pub mod app;
pub mod siri;

use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;

    let api_key = std::env::var("API_KEY")?;
    let gare = "45102"; // Chatelet les Halles
    let response = api::request(gare, &api_key)?;
    //println!("response: {}", response);
    let siri = api::parse_siri(&response)?;
    //println!("siri: {:?}", siri);
    let _ = app::pretty_print(&siri);
    Ok(())
}
