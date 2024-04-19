use reqwest::{
    self,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};

use serde::{Deserialize, Serialize};

use clap::Parser;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]

struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}
#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "TOKEN")]
    token: String,
    #[arg(short, long, value_name = "ARTIST")]
    artist: String,
}

fn tracks(tracks: Vec<&Track>) -> String {
    let mut result = String::new();

    for track in tracks {
        result.push_str(&format!("🎶 TITLE: {} \n", track.name));
        result.push_str(&format!("💿 ALBUM: {} \n" , track.album.name));
        result.push_str(&format!(
            "🕺 ARTIST: {} \n",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        ));
        result.push_str(&format!("🌎 LINK:{} \n", track.external_urls.spotify));
        result.push_str(&format!("------------------------------------------------------------------------------------------------------- \n"));
    }

    result
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = cli.artist
    );
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", cli.token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => {
                    let tracks_string = tracks(parsed.tracks.items.iter().collect());
                    println!("{}", tracks_string);
                }    
                Err(_) => println!("The response didn't match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        other => {
            panic!("Something unexpected happened : {:?}", other);
        }
    };
}



//cargo run -- --token <token> --artist "<nomeartista>" 
