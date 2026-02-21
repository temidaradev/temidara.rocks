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
struct JellyfinSession {
    #[serde(rename = "NowPlayingItem", default)]
    now_playing_item: Option<JellyfinItem>,
    #[serde(rename = "PlayState", default)]
    play_state: Option<JellyfinPlayState>,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct JellyfinItem {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Type")]
    media_type: String,
    #[serde(rename = "Album", default)]
    album: Option<String>,
    #[serde(rename = "Artists", default)]
    artists: Option<Vec<String>>,
    #[serde(rename = "AlbumArtists", default)]
    album_artists: Option<Vec<JellyfinArtistInfo>>,
    #[serde(rename = "ProductionYear", default)]
    production_year: Option<i32>,
    #[serde(rename = "Id")]
    id: String,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct JellyfinArtistInfo {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Id")]
    id: String,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct JellyfinPlayState {
    #[serde(rename = "IsPaused", default)]
    is_paused: Option<bool>,
}

#[server(GetJellyfinCurrentTrack, "/api")]
pub async fn get_jellyfin_current_track() -> Result<Option<TrackInfo>, ServerFnError> {
    use reqwest::Client;
    use std::env;

    let jellyfin_url = env::var("JELLYFIN_URL").unwrap_or_default();
    let jellyfin_token = env::var("JELLYFIN_TOKEN").unwrap_or_default();

    if jellyfin_url.is_empty() || jellyfin_token.is_empty() {
        eprintln!("[jellyfin] JELLYFIN_URL or JELLYFIN_TOKEN not set");
        return Ok(None);
    }

    let client = Client::new();
    let url = format!("{}/Sessions", jellyfin_url);

    // Use the modern Authorization: MediaBrowser header format
    let auth_header = format!(
        "MediaBrowser Client=\"temidara.rocks\", Device=\"Web\", DeviceId=\"temidara-portfolio\", Version=\"1.0.0\", Token=\"{}\"",
        jellyfin_token
    );

    let resp = client
        .get(&url)
        .header("Authorization", &auth_header)
        .header("Accept", "application/json")
        .send()
        .await;

    match resp {
        Ok(response) => {
            let status = response.status();
            if !status.is_success() {
                let body = response.text().await.unwrap_or_default();
                eprintln!(
                    "[jellyfin] API returned status {}: {}",
                    status,
                    &body[..body.len().min(300)]
                );
                return Ok(None);
            }

            let body = match response.text().await {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("[jellyfin] Failed to read response body: {}", e);
                    return Ok(None);
                }
            };

            let sessions: Vec<JellyfinSession> = match serde_json::from_str(&body) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("[jellyfin] Failed to parse sessions: {}", e);
                    eprintln!("[jellyfin] Response body: {}", &body[..body.len().min(500)]);
                    return Ok(None);
                }
            };

            eprintln!("[jellyfin] Found {} sessions", sessions.len());

            let track = sessions.into_iter().find(|s| {
                if let Some(item) = &s.now_playing_item {
                    eprintln!(
                        "[jellyfin] Session has item: {} (type: {})",
                        item.name, item.media_type
                    );
                    item.media_type == "Audio"
                } else {
                    false
                }
            });

            if let Some(session) = track {
                if let Some(item) = session.now_playing_item {
                    let is_paused = session
                        .play_state
                        .and_then(|ps| ps.is_paused)
                        .unwrap_or(false);

                    let status = if is_paused {
                        "paused".to_string()
                    } else {
                        "playing".to_string()
                    };

                    let thumb_url = format!(
                        "{}/Items/{}/Images/Primary?fillHeight=300&fillWidth=300&quality=96",
                        jellyfin_url, item.id
                    );

                    let artist = item
                        .artists
                        .as_ref()
                        .filter(|a| !a.is_empty())
                        .map(|a| a.join(", "))
                        .or_else(|| {
                            item.album_artists
                                .as_ref()
                                .filter(|a| !a.is_empty())
                                .map(|a| {
                                    a.iter()
                                        .map(|ai| ai.name.clone())
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                })
                        })
                        .unwrap_or_else(|| "Unknown Artist".to_string());

                    eprintln!(
                        "[jellyfin] Now playing: {} - {} ({})",
                        artist, item.name, status
                    );

                    return Ok(Some(TrackInfo {
                        title: item.name,
                        artist,
                        album: item.album.unwrap_or_else(|| "Unknown Album".to_string()),
                        year: item.production_year.map(|y| y.to_string()),
                        thumb_url: Some(thumb_url),
                        status,
                    }));
                }
            } else {
                eprintln!("[jellyfin] No audio session found");
            }
        }
        Err(e) => {
            eprintln!("[jellyfin] Request failed: {}", e);
        }
    }

    Ok(None)
}
