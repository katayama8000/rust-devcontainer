use dotenv::dotenv;
use gyazo_client::{GyazoClient, UploadParams};
use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub async fn run() {
    println!("gyazo.rs");
    dotenv().ok();

    let access_token =
        env::var("GYAZO_ACCESS_TOKEN").expect("GYAZO_ACCESS_TOKEN environment variable is not set");

    let gyazo_client = GyazoClient::new(access_token);
    let image_data = std::fs::read("img/moufu.png").expect("Failed to read file");
    let upload_params = UploadParams {
        imagedata: image_data,
        title: Some("moufu".to_string()),
        desc: Some("moufu is my cat".to_string()),
        access_policy: Some("public".to_string()),
        metadata_is_public: Some("true".to_string()),
        referer_url: Some("https://example.com".to_string()),
        app: None,
        created_at: None,
        collection_id: None,
    };

    let upload_response = gyazo_client.upload_image(upload_params).await.unwrap();
    println!("Image uploaded successfully!");
    println!("Image ID: {}", upload_response.image_id);

    // get image
    let image_id = &upload_response.image_id;
    let image = gyazo_client.get_image(image_id).await.unwrap();

    println!("Image ID: {}", image.image_id);

    // list images
    let images = gyazo_client.list_images().await.unwrap();
    for image in images {
        println!("Image ID: {}", image.image_id);
    }

    // delete image
    let delete_response = gyazo_client.delete_image(image_id).await.unwrap();
    println!("Image deleted successfully!");
}
