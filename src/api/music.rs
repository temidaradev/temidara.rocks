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
struct LbResponse {
    payload: LbPayload,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct LbPayload {
    count: usize,
    listens: Vec<LbListen>,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct LbListen {
    #[serde(default)]
    playing_now: bool,
    track_metadata: LbTrackMetadata,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct LbTrackMetadata {
    artist_name: String,
    release_name: Option<String>,
    track_name: String,
    #[serde(default)]
    mbid_mapping: Option<LbMbidMapping>,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct LbMbidMapping {
    release_mbid: Option<String>,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct MbResponse {
    releases: Vec<MbRelease>,
}

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Debug)]
struct MbRelease {
    id: String,
}

#[server(GetListenbrainzCurrentTrack, "/api")]
pub async fn get_listenbrainz_current_track() -> Result<Option<TrackInfo>, ServerFnError> {
    use reqwest::Client;
    use std::env;

    let username = env::var("LISTENBRAINZ_USERNAME").unwrap_or_else(|_| "temidaradev".to_string());
    let client = Client::new();

    let url = format!(
        "https://api.listenbrainz.org/1/user/{}/playing-now",
        username
    );
    eprintln!("[listenbrainz] Fetching from: {}", url);
    let resp = client
        .get(&url)
        .header("Accept", "application/json")
        .header(
            "User-Agent",
            "temidaradev-rocks/1.0.0 ( temidara@rocks.com )",
        )
        .send()
        .await;

    let mut is_playing = true;

    let body = match resp {
        Ok(response) => {
            if response.status().is_success() {
                let initial_body = response.text().await.unwrap_or_default();
                let lb_data_res = serde_json::from_str::<LbResponse>(&initial_body);

                if let Ok(lb_data) = lb_data_res {
                    if lb_data.payload.count == 0 {
                        let fallback_url = format!(
                            "https://api.listenbrainz.org/1/user/{}/listens?count=1",
                            username
                        );
                        eprintln!(
                            "[listenbrainz] No playing track, falling back to: {}",
                            fallback_url
                        );
                        let fallback_resp = client
                            .get(&fallback_url)
                            .header("Accept", "application/json")
                            .header(
                                "User-Agent",
                                "temidaradev-rocks/1.0.0 ( temidara@rocks.com )",
                            )
                            .send()
                            .await;

                        match fallback_resp {
                            Ok(fallback_response) if fallback_response.status().is_success() => {
                                is_playing = false;
                                fallback_response.text().await.unwrap_or_default()
                            }
                            Ok(fallback_response) => {
                                eprintln!(
                                    "[listenbrainz] Fallback API returned status {}",
                                    fallback_response.status()
                                );
                                return Ok(None);
                            }
                            Err(e) => {
                                eprintln!("[listenbrainz] Fallback request failed: {}", e);
                                return Ok(None);
                            }
                        }
                    } else {
                        initial_body
                    }
                } else {
                    eprintln!("[listenbrainz] Failed to parse playing-now payload");
                    initial_body
                }
            } else {
                eprintln!("[listenbrainz] API returned status {}", response.status());
                return Ok(None);
            }
        }
        Err(e) => {
            eprintln!("[listenbrainz] Request failed: {}", e);
            return Ok(None);
        }
    };

    if body.is_empty() {
        return Ok(None);
    }

    let lb_data: LbResponse = match serde_json::from_str(&body) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[listenbrainz] Failed to parse payload: {}", e);
            eprintln!(
                "[listenbrainz] Response body: {}",
                &body[..body.len().min(500)]
            );
            return Ok(None);
        }
    };

    if lb_data.payload.count == 0 || lb_data.payload.listens.is_empty() {
        eprintln!("[listenbrainz] No listens found");
        return Ok(None);
    }

    let listen = &lb_data.payload.listens[0];
    let meta = &listen.track_metadata;

    let track_status = if is_playing || listen.playing_now {
        "playing".to_string()
    } else {
        "paused".to_string()
    };

    let mut thumb_url = None;
    if let Some(ref mapping) = meta.mbid_mapping {
        if let Some(ref release_mbid) = mapping.release_mbid {
            thumb_url = Some(format!(
                "https://coverartarchive.org/release/{}/front",
                release_mbid
            ));
        }
    }

    if thumb_url.is_none() {
        use urlencoding::encode;
        let artist_name = &meta.artist_name;
        let track_name = &meta.track_name;
        let release_name = meta.release_name.as_deref().unwrap_or_default();

        if !release_name.is_empty() {
            let mb_url = format!(
                "https://musicbrainz.org/ws/2/release/?query=release:%22{}%22%20AND%20artist:%22{}%22&fmt=json",
                encode(release_name),
                encode(artist_name)
            );

            if let Ok(resp) = client
                .get(&mb_url)
                .header(
                    "User-Agent",
                    "temidaradev-rocks/1.0.0 ( temidara@rocks.com )",
                )
                .send()
                .await
            {
                if resp.status().is_success() {
                    if let Ok(mb_data) = resp.json::<MbResponse>().await {
                        if let Some(release) = mb_data.releases.first() {
                            thumb_url = Some(format!(
                                "https://coverartarchive.org/release/{}/front",
                                release.id
                            ));
                        }
                    }
                }
            }
        }

        if thumb_url.is_none() {
            let mb_url = format!(
                "https://musicbrainz.org/ws/2/recording/?query=recording:%22{}%22%20AND%20artist:%22{}%22&fmt=json",
                encode(track_name),
                encode(artist_name)
            );

            if let Ok(resp) = client
                .get(&mb_url)
                .header(
                    "User-Agent",
                    "temidaradev-rocks/1.0.0 ( temidara@rocks.com )",
                )
                .send()
                .await
            {
                if resp.status().is_success() {
                    if let Ok(json) = resp.json::<serde_json::Value>().await {
                        if let Some(recording) =
                            json["recordings"].as_array().and_then(|a| a.first())
                        {
                            if let Some(release) =
                                recording["releases"].as_array().and_then(|a| a.first())
                            {
                                if let Some(id) = release["id"].as_str() {
                                    thumb_url = Some(format!(
                                        "https://coverartarchive.org/release/{}/front",
                                        id
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    eprintln!(
        "[listenbrainz] Track: {} - {} ({})",
        meta.artist_name, meta.track_name, track_status
    );

    Ok(Some(TrackInfo {
        title: meta.track_name.clone(),
        artist: meta.artist_name.clone(),
        album: meta
            .release_name
            .clone()
            .unwrap_or_else(|| "Unknown Album".to_string()),
        year: None,
        thumb_url,
        status: track_status,
    }))
}
