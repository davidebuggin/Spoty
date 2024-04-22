use std::error::Error;

use clap::Parser;
use reqwest::{
    self,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};
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
    album: Album,
    href: String,
    popularity: u32,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Items {
    items: Vec<Track>,
}
#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items,
}
#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "TOKEN")]
    token: String,
    #[arg(short, long, value_name = "ARTIST")]
    artist: String,
}

fn tracks_into_string(tracks: Vec<Track>) -> String {
    let mut result = String::new();

    for track in tracks {
        result.push_str(&format!("🎶 TITLE: {} \n", track.name));
        result.push_str(&format!("💿 ALBUM: {} \n", track.album.name));
        result.push_str(&format!(
            "🕺 ARTIST: {} \n",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        ));
        result.push_str(&format!("🌎 LINK: {} \n", track.external_urls.spotify));
        result.push_str(&format!("------------------------------------------------------------------------------------------------------- \n"));
    }

    result
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let spotify_client = SpotifyClient {
        token: cli.token,
        client: reqwest::Client::new(),
    };

    match spotify_client.get_tracks(&cli.artist).await {
        Ok(tracks) => {
            let track_string = tracks_into_string(tracks.items);
            println!("{}", track_string);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

trait ClientSpotify {
    async fn get_tracks(&self, artist: &str) -> Result<Items, Box<dyn Error>>;
}

struct SpotifyClient {
    token: String,
    client: Client,
}

impl ClientSpotify for SpotifyClient {
    async fn get_tracks(&self, artist: &str) -> Result<Items, Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/search?q={query}&type=track,artist",
            query = artist
        );
        let response = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let parsed: APIResponse = response.json().await?;
                Ok(parsed.tracks)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Need to grab a new token");
                Err("Unauthorized".into())
            }
            other => {
                panic!("Something unexpected happened : {:?}", other);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_tracks() {
        let test_track = Track {
            name: "Song".to_string(),
            album: Album {
                name: "Album".to_string(),
                artists: vec![
                    Artist {
                        name: "Artist 1".to_string(),
                        external_urls: ExternalUrls {
                            spotify: "http://example.com".to_string(),
                        },
                    },
                    Artist {
                        name: "Artist 2".to_string(),
                        external_urls: ExternalUrls {
                            spotify: "http://example.com".to_string(),
                        },
                    },
                ],
                external_urls: ExternalUrls {
                    spotify: "http://example.com".to_string(),
                },
            },
            href: "http://example.com".to_string(),
            popularity: 1,
            external_urls: ExternalUrls {
                spotify: "http://example.com".to_string(),
            },
        };

        let result = tracks_into_string(vec![test_track]);

        let expected_result = "🎶 TITLE: Song \n💿 ALBUM: Album \n🕺 ARTIST: Artist 1, Artist 2 \n🌎 LINK: http://example.com \n------------------------------------------------------------------------------------------------------- \n";

        assert_eq!(result, expected_result);
    }
}

//cargo run -- --token <token> --artist "<nomeartista>"
