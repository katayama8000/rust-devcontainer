use supabase_rs::SupabaseClient;

use dotenv::dotenv;
use std::env::var;

async fn initialize_supabase_client() -> SupabaseClient {
    dotenv().ok(); // Load the .env file

    let supabase_client: SupabaseClient =
        SupabaseClient::new(var("SUPABASE_URL").unwrap(), var("SUPABASE_KEY").unwrap());

    supabase_client
}

pub async fn run() {
    println!("supabase.rs");
    let supabase_client = initialize_supabase_client().await;
    let response = supabase_client.select("dev_users").execute().await.unwrap();
    println!("{:?}", response);
}
