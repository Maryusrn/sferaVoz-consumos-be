use mongodb::{Client, options::ClientOptions};
use std::env;
use std::error::Error;

pub async fn establish_connection() -> Result<Client, Box<dyn Error>> {
    dotenv::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")?;
    
    let client_options = ClientOptions::parse(&database_url).await?;
    let client = Client::with_options(client_options)?;
    
    Ok(client)
}
