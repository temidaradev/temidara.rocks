use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: Option<String>,
    pub thumb_url: Option<String>,
    pub status: String,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct PlexSessionResponse {
    #[serde(rename = "MediaContainer")]
    media_container: MediaContainer,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct MediaContainer {
    #[serde(rename = "size")]
    size: i32,
    #[serde(rename = "Metadata", default)]
    metadata: Vec<Metadata>,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    #[serde(rename = "type")]
    media_type: String,
    title: String,
    #[serde(rename = "grandparentTitle")]
    artist: Option<String>,
    #[serde(rename = "parentTitle")]
    album: Option<String>,
    year: Option<i32>,
    thumb: Option<String>,
    #[serde(rename = "User")]
    user: Option<User>,
    #[serde(rename = "Player")]
    player: Option<Player>,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct User {
    title: String,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct Player {
    state: String,
}

#[server]
pub async fn get_plex_current_track() -> Result<Option<TrackInfo>, ServerFnError> {
    use std::env;
    use reqwest::Client;

    let plex_url = env::var("PLEX_URL").unwrap_or_default();
    let plex_token = env::var("PLEX_TOKEN").unwrap_or_default();
    
    if plex_url.is_empty() || plex_token.is_empty() {
        return Ok(None);
    }

    let client = Client::new();
    let url = format!("{}/status/sessions", plex_url);

    let resp = client
        .get(&url)
        .header("X-Plex-Token", plex_token)
        .header("Accept", "application/json")
        .send()
        .await;

    match resp {
        Ok(response) => {
            if response.status().is_success() {
                let body_text = response.text().await?;
                let sessions: PlexSessionResponse = serde_json::from_str(&body_text)?;
                
                let track = sessions.media_container.metadata.into_iter().find(|m| {
                     let is_track = m.media_type == "track";
                     let state = m.player.as_ref().map(|p| p.state.as_str()).unwrap_or("stopped");
                     let is_active = state == "playing" || state == "paused";
                     is_track && is_active
                });

                if let Some(t) = track {
                     let status = t.player.as_ref().map(|p| p.state.clone()).unwrap_or_else(|| "unknown".to_string());
                     
                     let full_thumb_url = t.thumb.map(|thumb_path| {
                         format!("{}{}?X-Plex-Token={}", plex_url, thumb_path, env::var("PLEX_TOKEN").unwrap_or_default())
                     });
                     
                     return Ok(Some(TrackInfo {
                         title: t.title,
                         artist: t.artist.unwrap_or_else(|| "Unknown Artist".to_string()),
                         album: t.album.unwrap_or_else(|| "Unknown Album".to_string()),
                         year: t.year.map(|y: i32| y.to_string()),
                         thumb_url: full_thumb_url,
                         status,
                     }));
                }
            }
        }
        Err(_) => {}
    }

    Ok(None)
}
