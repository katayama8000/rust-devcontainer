use dotenv::dotenv;
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

    let file_path = "img/moufu.png";

    let mut file = File::open(file_path).expect("Failed to open file");
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content)
        .expect("Failed to read file");

    match upload_image(access_token.as_str(), file_content).await {
        Ok(response) => println!("Upload response: {:?}", response),
        Err(err) => eprintln!("Error uploading image: {}", err),
    }

    // match list_images(&access_token).await {
    //     Ok(images) => println!("Images: {:#?}", images),
    //     Err(err) => eprintln!("Failed to list images: {}", err),
    // }

    // let image_id = "95bece562072a4bf3ef4415bb723b1e7";
    // match get_image(&access_token, image_id).await {
    //     Ok(image_response) => println!(
    //         "Image downloaded successfully! Parsed response: {:#?}",
    //         image_response
    //     ),
    //     Err(err) => eprintln!("Failed to download image: {}", err),
    // }

    // let image_id = "95bece562072a4bf3ef4415bb723b1e7";
    // match delete_image(&access_token, image_id).await {
    //     Ok(delete_response) => println!(
    //         "Image deleted successfully! Parsed response: {:#?}",
    //         delete_response
    //     ),
    //     Err(err) => eprintln!("Failed to delete image: {}", err),
    // }
}

async fn upload_image(
    access_token: &str,
    image_data: Vec<u8>,
) -> Result<UploadImageResponse, Box<dyn Error>> {
    let client = Client::new();

    let form = Form::new()
        .part("imagedata", Part::bytes(image_data).file_name("image.png"))
        .text("metadata_is_public", "true")
        .text("title", "cat_is_my_life");

    let response = client
        .post(format!(
            "https://upload.gyazo.com/api/upload?access_token={}",
            access_token
        ))
        .multipart(form)
        .send()
        .await?;

    if response.status().is_success() {
        let upload_response: UploadImageResponse = response.json().await?;
        println!("Image uploaded successfully!");
        Ok(upload_response)
    } else {
        Err(format!("Failed to upload image. Status: {}", response.status()).into())
    }
}

async fn list_images(access_token: &str) -> Result<Vec<GyazoImageResponse>, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(format!(
            "https://api.gyazo.com/api/images?access_token={}",
            access_token
        ))
        .send()
        .await?;

    if response.status().is_success() {
        let images: Vec<GyazoImageResponse> = response.json().await?;
        Ok(images)
    } else {
        Err(format!("Failed to get images. Status: {}", response.status()).into())
    }
}

async fn get_image(
    access_token: &str,
    image_id: &str,
) -> Result<GyazoImageResponse, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(format!(
            "https://api.gyazo.com/api/images/{}?access_token={}",
            image_id, access_token
        ))
        .send()
        .await?;

    if response.status().is_success() {
        let gyazo_response: GyazoImageResponse = response.json().await?;
        Ok(gyazo_response)
    } else {
        Err(format!("Failed to download image. Status: {}", response.status()).into())
    }
}

async fn delete_image(
    access_token: &str,
    image_id: &str,
) -> Result<DeleteImageResponse, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://api.gyazo.com/api/images/{}", image_id);

    let response = client.delete(&url).bearer_auth(access_token).send().await?;

    match response.status() {
        reqwest::StatusCode::NO_CONTENT => {
            println!("Image deleted successfully!");
        }
        status => {
            eprintln!("Failed to delete image. Status: {}", status);
        }
    }

    let delete_image_response = response.json::<DeleteImageResponse>().await?;
    Ok(delete_image_response)
}

#[derive(Debug, Deserialize)]
struct GyazoImageResponse {
    image_id: String,
    permalink_url: Option<String>,
    thumb_url: Option<String>,
    #[serde(rename = "type")]
    image_type: String,
    created_at: String,
    metadata: ImageMetadata,
    ocr: Option<ImageOcr>,
}

#[derive(Debug, Deserialize)]
struct ImageMetadata {
    app: Option<String>,
    title: Option<String>,
    url: Option<String>,
    desc: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ImageOcr {
    locale: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct UploadImageResponse {
    image_id: String,
    permalink_url: String,
    thumb_url: String,
    url: String,
    #[serde(rename = "type")]
    image_type: String,
}

#[derive(Debug, Deserialize)]
struct DeleteImageResponse {
    image_id: String,
    #[serde(rename = "type")]
    image_type: String,
}
