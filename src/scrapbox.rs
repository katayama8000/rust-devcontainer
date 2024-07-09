use chrono::{Datelike, Duration, Local, Weekday};
use dotenv::dotenv;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Client;
use scraper::{Html, Selector};
use std::env;
use std::error::Error;
use url::Url;

const DAILY_TEXT: &str = "[* ルーティン]\n[* 感想]\n#daily";
const WEEKLY_TEXT: &str = "[* 目標]\n[* 振り返り]\n[* 感想]\n[* 日記]\n#weekly";

async fn check_page_exists(
    client: &Client,
    project: &str,
    title: &str,
) -> Result<bool, Box<dyn Error>> {
    let encoded_title = utf8_percent_encode(title, NON_ALPHANUMERIC).to_string();
    let url = format!("https://scrapbox.io/{}/{}", project, encoded_title);
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("div.error-message").unwrap();
    Ok(!document.select(&selector).next().is_some())
}

async fn create_scrapbox_page(client: &Client, url: &str) -> Result<(), Box<dyn Error>> {
    client.get(url).send().await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    Ok(())
}

async fn write_to_scrapbox(
    sid: &str,
    project: &str,
    title: &str,
    text: &str,
) -> Result<(), Box<dyn Error>> {
    let encoded_title = utf8_percent_encode(title, NON_ALPHANUMERIC).to_string();
    let url = Url::parse_with_params(
        &format!("https://scrapbox.io/{}/{}", project, encoded_title),
        &[("body", text)],
    )?;

    let client = Client::builder().build()?;

    client
        .get("https://scrapbox.io")
        .header("Cookie", format!("connect.sid={}", sid))
        .send()
        .await?;

    if check_page_exists(&client, project, title).await? {
        println!("Page \"{}\" already exists.", title);
        return Ok(());
    }

    create_scrapbox_page(&client, url.as_str()).await?;
    Ok(())
}

fn generate_titles() -> (String, String) {
    let today = Local::now().naive_local().date();
    let year = today.year();
    let month = today.month();
    let date = today.day();
    let day = match today.weekday() {
        Weekday::Sun => "Sun",
        Weekday::Mon => "Mon",
        Weekday::Tue => "Tue",
        Weekday::Wed => "Wed",
        Weekday::Thu => "Thu",
        Weekday::Fri => "Fri",
        Weekday::Sat => "Sat",
    };

    let daily_title = format!("Rust {}/{}/{} ({})", year, month, date, day);
    let weekly_end = today + Duration::days(6);
    let weekly_title = format!(
        "{}/{}/{} ~ {}/{}/{}",
        year,
        month,
        date,
        weekly_end.year(),
        weekly_end.month(),
        weekly_end.day()
    );

    (daily_title, weekly_title)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <daily|weekly>");
        std::process::exit(1);
    }

    let template = &args[1];
    let (daily_title, weekly_title) = generate_titles();
    let (title, text) = if template == "daily" {
        (daily_title, DAILY_TEXT)
    } else {
        (weekly_title, WEEKLY_TEXT)
    };

    let sid = env::var("SCRAPBOX_SID").expect("SCRAPBOX_SID must be set");

    println!("Writing to Scrapbox: {}...", title);
    write_to_scrapbox(&sid, "katayama8000", &title, text).await?;
    println!("Done!");

    Ok(())
}

pub fn run() {
    println!("scrapbox.rs");
    match main() {
        Ok(_) => println!("Ok"),
        Err(e) => println!("Err: {:?}", e),
    }
}
