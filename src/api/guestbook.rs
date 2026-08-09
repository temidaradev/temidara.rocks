use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestbookEntry {
    pub id: u64,
    pub name: String,
    pub message: String,
    pub created_at: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmissionResult {
    pub pending: bool,
}

#[cfg(feature = "ssr")]
#[derive(Clone, Serialize, Deserialize)]
struct StoredGuestbookEntry {
    id: u64,
    name: String,
    message: String,
    created_at: u64,
    approved: bool,
}

#[cfg(feature = "ssr")]
fn store_lock() -> &'static std::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| std::sync::Mutex::new(()))
}

#[cfg(feature = "ssr")]
fn rate_limits(
) -> &'static std::sync::Mutex<std::collections::HashMap<String, Vec<std::time::Instant>>> {
    static LIMITS: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashMap<String, Vec<std::time::Instant>>>,
    > = std::sync::OnceLock::new();
    LIMITS.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

#[cfg(feature = "ssr")]
fn guestbook_path() -> std::path::PathBuf {
    std::env::var("GUESTBOOK_PATH")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("data/guestbook.json"))
}

#[cfg(feature = "ssr")]
fn read_entries() -> Result<Vec<StoredGuestbookEntry>, ServerFnError> {
    let path = guestbook_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let contents = std::fs::read_to_string(path)
        .map_err(|error| ServerFnError::new(format!("failed to read guestbook: {error}")))?;
    serde_json::from_str(&contents)
        .map_err(|error| ServerFnError::new(format!("failed to parse guestbook: {error}")))
}

#[cfg(feature = "ssr")]
fn write_entries(entries: &[StoredGuestbookEntry]) -> Result<(), ServerFnError> {
    let path = guestbook_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            ServerFnError::new(format!("failed to create guestbook storage: {error}"))
        })?;
    }
    let temporary = path.with_extension("json.tmp");
    let contents = serde_json::to_string_pretty(entries)
        .map_err(|error| ServerFnError::new(format!("failed to serialize guestbook: {error}")))?;
    std::fs::write(&temporary, contents)
        .map_err(|error| ServerFnError::new(format!("failed to write guestbook: {error}")))?;
    std::fs::rename(temporary, path)
        .map_err(|error| ServerFnError::new(format!("failed to save guestbook: {error}")))
}

#[cfg(feature = "ssr")]
fn validate_submission(name: &str, message: &str) -> Result<(), ServerFnError> {
    let name_length = name.chars().count();
    let message_length = message.chars().count();
    if !(1..=40).contains(&name_length) {
        return Err(ServerFnError::new(
            "name must be between 1 and 40 characters",
        ));
    }
    if !(2..=400).contains(&message_length) {
        return Err(ServerFnError::new(
            "message must be between 2 and 400 characters",
        ));
    }
    if name.chars().any(char::is_control)
        || message
            .chars()
            .any(|character| character.is_control() && !character.is_whitespace())
    {
        return Err(ServerFnError::new(
            "message contains unsupported characters",
        ));
    }
    let link_count = message.matches("http://").count() + message.matches("https://").count();
    if link_count > 1 {
        return Err(ServerFnError::new("only one link is allowed"));
    }
    Ok(())
}

#[cfg(feature = "ssr")]
async fn visitor_key() -> String {
    let headers: axum::http::HeaderMap = leptos_axum::extract().await.unwrap_or_default();
    for header in ["cf-connecting-ip", "x-forwarded-for", "x-real-ip"] {
        if let Some(value) = headers.get(header).and_then(|value| value.to_str().ok()) {
            return value.split(',').next().unwrap_or(value).trim().to_string();
        }
    }
    headers
        .get("user-agent")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown")
        .to_string()
}

#[cfg(feature = "ssr")]
fn enforce_rate_limit(key: String) -> Result<(), ServerFnError> {
    let mut limits = rate_limits()
        .lock()
        .map_err(|_| ServerFnError::new("rate limiter unavailable"))?;
    let now = std::time::Instant::now();
    let attempts = limits.entry(key).or_default();
    attempts.retain(|attempt| now.duration_since(*attempt) < std::time::Duration::from_secs(3600));
    if attempts.len() >= 3 {
        return Err(ServerFnError::new("too many messages; try again later"));
    }
    attempts.push(now);
    Ok(())
}

#[cfg(feature = "ssr")]
fn require_admin(token: &str) -> Result<(), ServerFnError> {
    let configured = std::env::var("GUESTBOOK_ADMIN_TOKEN")
        .map_err(|_| ServerFnError::new("guestbook moderation is not configured"))?;
    if configured.is_empty() || token.as_bytes() != configured.as_bytes() {
        return Err(ServerFnError::new("invalid moderation token"));
    }
    Ok(())
}

#[server(ListGuestbookEntries, "/api")]
pub async fn list_guestbook_entries() -> Result<Vec<GuestbookEntry>, ServerFnError> {
    let _guard = store_lock()
        .lock()
        .map_err(|_| ServerFnError::new("guestbook unavailable"))?;
    let mut entries = read_entries()?
        .into_iter()
        .filter(|entry| entry.approved)
        .map(|entry| GuestbookEntry {
            id: entry.id,
            name: entry.name,
            message: entry.message,
            created_at: entry.created_at,
        })
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| std::cmp::Reverse(entry.created_at));
    entries.truncate(50);
    Ok(entries)
}

#[server(SubmitGuestbookEntry, "/api")]
pub async fn submit_guestbook_entry(
    name: String,
    message: String,
    website: String,
) -> Result<SubmissionResult, ServerFnError> {
    if !website.trim().is_empty() {
        return Ok(SubmissionResult { pending: true });
    }

    let name = name.trim().to_string();
    let message = message.trim().to_string();
    validate_submission(&name, &message)?;
    enforce_rate_limit(visitor_key().await)?;

    let approved = std::env::var("GUESTBOOK_AUTO_APPROVE")
        .is_ok_and(|value| value.eq_ignore_ascii_case("true"));
    let created_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let _guard = store_lock()
        .lock()
        .map_err(|_| ServerFnError::new("guestbook unavailable"))?;
    let mut entries = read_entries()?;
    let id = entries
        .iter()
        .map(|entry| entry.id)
        .max()
        .unwrap_or(0)
        .saturating_add(1)
        .max(created_at);
    entries.push(StoredGuestbookEntry {
        id,
        name,
        message,
        created_at,
        approved,
    });
    write_entries(&entries)?;

    Ok(SubmissionResult { pending: !approved })
}

#[server(ListPendingGuestbookEntries, "/api")]
pub async fn list_pending_guestbook_entries(
    token: String,
) -> Result<Vec<GuestbookEntry>, ServerFnError> {
    require_admin(&token)?;
    let _guard = store_lock()
        .lock()
        .map_err(|_| ServerFnError::new("guestbook unavailable"))?;
    let mut entries = read_entries()?
        .into_iter()
        .filter(|entry| !entry.approved)
        .map(|entry| GuestbookEntry {
            id: entry.id,
            name: entry.name,
            message: entry.message,
            created_at: entry.created_at,
        })
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.created_at);
    Ok(entries)
}

#[server(ModerateGuestbookEntry, "/api")]
pub async fn moderate_guestbook_entry(
    token: String,
    id: u64,
    action: String,
) -> Result<(), ServerFnError> {
    require_admin(&token)?;
    let _guard = store_lock()
        .lock()
        .map_err(|_| ServerFnError::new("guestbook unavailable"))?;
    let mut entries = read_entries()?;
    match action.as_str() {
        "approve" => {
            let entry = entries
                .iter_mut()
                .find(|entry| entry.id == id)
                .ok_or_else(|| ServerFnError::new("entry not found"))?;
            entry.approved = true;
        }
        "delete" => entries.retain(|entry| entry.id != id),
        _ => return Err(ServerFnError::new("invalid moderation action")),
    }
    write_entries(&entries)
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    #[test]
    fn validates_guestbook_limits() {
        assert!(validate_submission("Ada", "Hello there").is_ok());
        assert!(validate_submission("", "Hello there").is_err());
        assert!(validate_submission("Ada", "x").is_err());
        assert!(validate_submission("Ada", "https://one.test https://two.test").is_err());
    }
}
