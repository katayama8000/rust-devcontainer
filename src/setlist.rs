use reqwest::header;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Setlists {
    #[serde(rename = "setlist")]
    setlist: Vec<Setlist>,
}

#[derive(Deserialize, Debug)]
struct Setlist {
    artist: Artist,
    venue: Venue,
    sets: Sets,
    eventDate: String,
}

#[derive(Deserialize, Debug)]
struct Artist {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Venue {
    name: String,
    city: City,
}

#[derive(Deserialize, Debug)]
struct City {
    name: String,
    country: Country,
}

#[derive(Deserialize, Debug)]
struct Country {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Sets {
    #[serde(rename = "set")]
    set: Vec<Set>,
}

#[derive(Deserialize, Debug)]
struct Set {
    song: Vec<Song>,
}

#[derive(Deserialize, Debug)]
struct Song {
    name: String,
}

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "";

    let url = format!("https://api.setlist.fm/rest/1.0/search/setlists?artistName=one%20ok%20rock");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(header::ACCEPT, "application/json")
        .header("x-api-key", api_key)
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        // println!("{}", body); // JSONレスポンス全体を出力 (デバッグ用)

        let setlists: Result<Setlists, serde_json::Error> = serde_json::from_str(&body);

        match setlists {
            Ok(data) => {
                println!("{}の過去のセットリスト:", data.setlist[0].artist.name);
                for setlist in data.setlist {
                    println!("  - 日付: {}", setlist.eventDate);
                    println!(
                        "    会場: {}, {} ({})",
                        setlist.venue.name,
                        setlist.venue.city.name,
                        setlist.venue.city.country.name
                    );
                    if !setlist.sets.set.is_empty() {
                        println!("    セットリスト:");
                        for set in setlist.sets.set {
                            for song in set.song {
                                println!("      - {}", song.name);
                            }
                        }
                    } else {
                        println!("    セットリスト情報なし");
                    }
                    println!("---");
                }
            }
            Err(e) => println!("JSONのパースに失敗: {}", e),
        }
    } else {
        println!("APIリクエスト失敗: {:?}", response.status());
    }

    Ok(())
}
