use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: Option<String>,
    pub release_mbid: Option<String>,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverArtQuery {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub release_mbid: Option<String>,
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
    additional_info: Option<LbAdditionalInfo>,
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
struct LbAdditionalInfo {
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

#[cfg(feature = "ssr")]
const MUSIC_USER_AGENT: &str = "temidaradev-rocks/1.0.0 (temidara@rocks.com)";

#[cfg(feature = "ssr")]
#[derive(Clone)]
struct CachedCover {
    url: Option<String>,
    cached_at: std::time::Instant,
}

#[cfg(feature = "ssr")]
fn cover_cache() -> &'static std::sync::Mutex<std::collections::HashMap<String, CachedCover>> {
    static CACHE: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashMap<String, CachedCover>>,
    > = std::sync::OnceLock::new();
    CACHE.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

#[cfg(feature = "ssr")]
fn cached_cover(key: &str) -> Option<Option<String>> {
    let mut cache = cover_cache().lock().ok()?;
    if let Some(entry) = cache.get(key) {
        let ttl = if entry.url.is_some() {
            std::time::Duration::from_secs(24 * 60 * 60)
        } else {
            std::time::Duration::from_secs(15 * 60)
        };
        if entry.cached_at.elapsed() < ttl {
            return Some(entry.url.clone());
        }
    }
    cache.remove(key);
    None
}

#[cfg(feature = "ssr")]
fn cache_cover(key: String, url: Option<String>) {
    if let Ok(mut cache) = cover_cache().lock() {
        cache.insert(
            key,
            CachedCover {
                url,
                cached_at: std::time::Instant::now(),
            },
        );
    }
}

#[cfg(feature = "ssr")]
async fn cover_for_release(client: &reqwest::Client, release_id: &str) -> Option<String> {
    let response = client
        .get(format!(
            "https://coverartarchive.org/release/{}",
            release_id
        ))
        .header("Accept", "application/json")
        .send()
        .await
        .ok()?;

    if !response.status().is_success() {
        return None;
    }

    let json = response.json::<serde_json::Value>().await.ok()?;
    let images = json["images"].as_array()?;
    let image = images
        .iter()
        .find(|image| image["front"].as_bool() == Some(true))
        .or_else(|| images.first())?;
    let url = image["thumbnails"]["250"]
        .as_str()
        .or_else(|| image["thumbnails"]["500"].as_str())?;

    Some(url.replacen("http://", "https://", 1))
}

#[cfg(feature = "ssr")]
async fn first_available_cover(
    client: &reqwest::Client,
    release_ids: Vec<String>,
    checked_ids: &mut std::collections::HashSet<String>,
) -> Option<String> {
    for release_id in release_ids {
        if checked_ids.insert(release_id.clone()) {
            if let Some(url) = cover_for_release(client, &release_id).await {
                return Some(url);
            }
        }
    }
    None
}

#[cfg(feature = "ssr")]
async fn resolve_cover_art(client: &reqwest::Client, query: &CoverArtQuery) -> Option<String> {
    use urlencoding::encode;

    let cache_key = if query.album.is_empty() {
        format!("{}\0{}", query.artist, query.title)
    } else {
        format!("{}\0{}", query.artist, query.album)
    }
    .to_lowercase();

    if let Some(cached) = cached_cover(&cache_key) {
        return cached;
    }

    let mut checked_ids = std::collections::HashSet::new();
    let mapped_release = query.release_mbid.clone().into_iter().collect::<Vec<_>>();

    if let Some(url) = first_available_cover(client, mapped_release, &mut checked_ids).await {
        cache_cover(cache_key, Some(url.clone()));
        return Some(url);
    }

    if !query.album.is_empty() {
        let url = format!(
            "https://musicbrainz.org/ws/2/release/?query=release:%22{}%22%20AND%20artist:%22{}%22&fmt=json&limit=5",
            encode(&query.album),
            encode(&query.artist)
        );
        if let Ok(response) = client
            .get(url)
            .header("Accept", "application/json")
            .send()
            .await
        {
            if response.status().is_success() {
                if let Ok(result) = response.json::<MbResponse>().await {
                    let release_ids = result
                        .releases
                        .into_iter()
                        .map(|release| release.id)
                        .collect();
                    if let Some(url) =
                        first_available_cover(client, release_ids, &mut checked_ids).await
                    {
                        cache_cover(cache_key, Some(url.clone()));
                        return Some(url);
                    }
                }
            }
        }
    }

    let url = format!(
        "https://musicbrainz.org/ws/2/recording/?query=recording:%22{}%22%20AND%20artist:%22{}%22&fmt=json&limit=3",
        encode(&query.title),
        encode(&query.artist)
    );
    if let Ok(response) = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await
    {
        if response.status().is_success() {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                let release_ids = json["recordings"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .flat_map(|recording| recording["releases"].as_array().into_iter().flatten())
                    .filter_map(|release| release["id"].as_str().map(str::to_string))
                    .take(5)
                    .collect();
                if let Some(url) =
                    first_available_cover(client, release_ids, &mut checked_ids).await
                {
                    cache_cover(cache_key, Some(url.clone()));
                    return Some(url);
                }
            }
        }
    }

    cache_cover(cache_key, None);
    None
}

#[cfg(feature = "ssr")]
fn cover_cache_key(query: &CoverArtQuery) -> String {
    use std::hash::{Hash, Hasher};

    let value = if query.album.is_empty() {
        format!("{}\0{}", query.artist, query.title)
    } else {
        format!("{}\0{}", query.artist, query.album)
    }
    .to_lowercase();
    let mut hasher = std::hash::DefaultHasher::new();
    value.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

#[cfg(feature = "ssr")]
fn cover_cache_dir() -> std::path::PathBuf {
    std::env::var("COVER_CACHE_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("data/covers"))
}

#[cfg(feature = "ssr")]
fn cached_cover_image_url(query: &CoverArtQuery) -> Option<String> {
    let key = cover_cache_key(query);
    cover_cache_dir()
        .join(format!("{key}.img"))
        .exists()
        .then(|| format!("/cover-cache/{key}.img"))
}

#[cfg(feature = "ssr")]
async fn cache_cover_image(
    client: &reqwest::Client,
    query: &CoverArtQuery,
    remote_url: &str,
) -> Option<String> {
    let key = cover_cache_key(query);
    let directory = cover_cache_dir();
    let image_path = directory.join(format!("{key}.img"));
    let mime_path = directory.join(format!("{key}.mime"));
    let local_url = format!("/cover-cache/{key}.img");

    if image_path.exists() {
        return Some(local_url);
    }

    let response = client.get(remote_url).send().await.ok()?;
    if !response.status().is_success() {
        return None;
    }
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .filter(|value| value.starts_with("image/"))
        .unwrap_or("image/jpeg")
        .to_string();
    let bytes = response.bytes().await.ok()?;
    if bytes.is_empty() || bytes.len() > 2 * 1024 * 1024 {
        return None;
    }

    std::fs::create_dir_all(&directory).ok()?;
    let temporary = directory.join(format!("{key}.tmp"));
    std::fs::write(&temporary, bytes).ok()?;
    std::fs::rename(temporary, image_path).ok()?;
    let _ = std::fs::write(mime_path, content_type);
    Some(local_url)
}

#[server(GetCoverArt, "/api")]
pub async fn get_cover_art(query: CoverArtQuery) -> Result<Option<String>, ServerFnError> {
    if let Some(url) = cached_cover_image_url(&query) {
        return Ok(Some(url));
    }

    let client = reqwest::Client::builder()
        .user_agent(MUSIC_USER_AGENT)
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .map_err(|error| ServerFnError::new(format!("failed to create HTTP client: {error}")))?;

    if let Some(release_mbid) = query.release_mbid.as_ref() {
        let direct_url = format!("https://coverartarchive.org/release/{release_mbid}/front-250");
        if let Some(url) = cache_cover_image(&client, &query, &direct_url).await {
            return Ok(Some(url));
        }
    }

    let Some(remote_url) = tokio::time::timeout(
        std::time::Duration::from_secs(4),
        resolve_cover_art(&client, &query),
    )
    .await
    .ok()
    .flatten() else {
        return Ok(None);
    };
    Ok(Some(
        cache_cover_image(&client, &query, &remote_url)
            .await
            .unwrap_or(remote_url),
    ))
}

#[server(GetListenbrainzCurrentTrack, "/api")]
pub async fn get_listenbrainz_current_track() -> Result<Option<TrackInfo>, ServerFnError> {
    use reqwest::Client;
    use std::env;

    let username = env::var("LISTENBRAINZ_USERNAME").unwrap_or_else(|_| "temidaradev".to_string());
    let client = Client::builder()
        .user_agent(MUSIC_USER_AGENT)
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|error| ServerFnError::new(format!("failed to create HTTP client: {error}")))?;

    let url = format!(
        "https://api.listenbrainz.org/1/user/{}/playing-now",
        username
    );
    let resp = client
        .get(&url)
        .header("Accept", "application/json")
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
                        let fallback_resp = client
                            .get(&fallback_url)
                            .header("Accept", "application/json")
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

    let release_mbid = meta
        .additional_info
        .as_ref()
        .and_then(|info| info.release_mbid.clone())
        .or_else(|| {
            meta.mbid_mapping
                .as_ref()
                .and_then(|mapping| mapping.release_mbid.clone())
        });

    Ok(Some(TrackInfo {
        title: meta.track_name.clone(),
        artist: meta.artist_name.clone(),
        album: meta
            .release_name
            .clone()
            .unwrap_or_else(|| "Unknown Album".to_string()),
        year: None,
        release_mbid,
        status: track_status,
    }))
}
