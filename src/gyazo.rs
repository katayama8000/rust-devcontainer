use dotenv::dotenv;
use reqwest::multipart::{Form, Part};
use reqwest::Client;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub async fn run() {
    println!("gyazo.rs");
    dotenv().ok();

    let access_token =
        env::var("GYAZO_ACCESS_TOKEN").expect("GYAZO_ACCESS_TOKEN environment variable is not set");
    let file_path = "img/moufu.png";
    // if let Err(err) = upload(access_token.as_str(), file_path).await {
    //     eprintln!("Failed to upload image: {}", err);
    // }

    // Gyazoから画像を取得する
    match get_image(access_token.as_str()).await {
        Ok(_) => println!("Image downloaded successfully!"),
        Err(err) => eprintln!("Failed to download image: {}", err),
    }
}

async fn upload(access_token: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    // ファイルが存在するか確認
    if !Path::new(file_path).exists() {
        println!("Error: The file does not exist at the specified path.");
        return Ok(());
    }

    // ファイルの内容を読み込む
    let mut file = File::open(file_path)?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content)?;

    // クライアントを作成
    let client = Client::new();

    // ファイルの内容をマルチパートフォームデータに追加
    let form = Form::new().part(
        "imagedata",
        Part::bytes(file_content).file_name("moufu.png"),
    );

    // リクエストを送信
    let response = client
        .post(format!(
            "https://upload.gyazo.com/api/upload?access_token={}",
            access_token
        ))
        .multipart(form)
        .send()
        .await?;

    // レスポンスを表示
    if response.status().is_success() {
        println!("Image uploaded successfully!");
        let response_text = response.text().await?;
        println!("Response: {}", response_text);
    } else {
        println!("Failed to upload image. Status: {}", response.status());
        let response_text = response.text().await?;
        println!("Response: {}", response_text);
    }

    Ok(())
}

async fn get_image(access_token: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Gyazoから画像を取得するリクエストを送信
    let response = client
        .get(format!(
            "https://api.gyazo.com/api/images?access_token={}",
            access_token
        ))
        .send()
        .await?;

    // レスポンスを確認して画像データを取得
    if let Ok(response_text) = response.text().await {
        println!("Response text: {:#?}", response_text);
    } else {
        println!("Failed to get response text");
    }
    Ok(())
}
