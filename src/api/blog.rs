use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct BlogInfo {
    pub title: String,
    pub text: String,
}

#[server]
pub async fn get_repo_readme(path: String) -> Result<String, ServerFnError> {
    let text = reqwest::get(path)
        .await?;

    Ok(text.text().await?)
}
