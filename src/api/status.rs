use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerStatus {
    pub uptime_seconds: Option<u64>,
    pub operating_system: String,
    pub architecture: String,
}

#[cfg(feature = "ssr")]
fn command_output(command: &str, arguments: &[&str]) -> Option<String> {
    std::process::Command::new(command)
        .args(arguments)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|output| output.trim().to_string())
        .filter(|output| !output.is_empty())
}

#[cfg(all(feature = "ssr", target_os = "linux"))]
fn system_uptime() -> Option<u64> {
    std::fs::read_to_string("/proc/uptime")
        .ok()?
        .split_whitespace()
        .next()?
        .parse::<f64>()
        .ok()
        .map(|seconds| seconds as u64)
}

#[cfg(all(feature = "ssr", target_os = "macos"))]
fn system_uptime() -> Option<u64> {
    let boot_time = command_output("sysctl", &["-n", "kern.boottime"])?;
    let seconds = boot_time
        .split("sec =")
        .nth(1)?
        .split(',')
        .next()?
        .trim()
        .parse::<u64>()
        .ok()?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();
    Some(now.saturating_sub(seconds))
}

#[cfg(all(feature = "ssr", not(any(target_os = "linux", target_os = "macos"))))]
fn system_uptime() -> Option<u64> {
    None
}

#[server(GetServerStatus, "/api")]
pub async fn get_server_status() -> Result<ServerStatus, ServerFnError> {
    Ok(ServerStatus {
        uptime_seconds: system_uptime(),
        operating_system: command_output("uname", &["-s"])
            .unwrap_or_else(|| std::env::consts::OS.to_string())
            .to_lowercase(),
        architecture: command_output("uname", &["-m"])
            .unwrap_or_else(|| std::env::consts::ARCH.to_string()),
    })
}
