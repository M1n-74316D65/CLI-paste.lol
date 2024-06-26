use std::collections::HashMap;

#[tokio::main]
pub async fn create(
    user: String,
    api_key: String,
    unlist: bool,
    title: String,
    content: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}{}{}", "https://api.omg.lol/address/", user, "/pastebin/");
    let mut map = HashMap::new();
    map.insert("title", title);
    map.insert("content", content);
    if !unlist {
        map.insert("listed", "".to_string());
    }
    let client: serde_json::Value = reqwest::Client::new()
        .post(url)
        .bearer_auth(api_key)
        .json(&map)
        .send()
        .await?
        .json()
        .await?;
    Ok(client)
}

#[tokio::main]
pub async fn remove(
    user: String,
    api_key: String,
    title_as_url: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!(
        "{}{}{}{}",
        "https://api.omg.lol/address/", user, "/pastebin/", title_as_url
    );
    let client: serde_json::Value = reqwest::Client::new()
        .delete(url)
        .bearer_auth(api_key)
        .send()
        .await?
        .json()
        .await?;
    Ok(client)
}

#[tokio::main]
pub async fn download(
    user: String,
    title_as_url: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!(
        "{}{}{}{}",
        "https://api.omg.lol/address/", user, "/pastebin/", title_as_url
    );
    let client: serde_json::Value = reqwest::Client::new().get(url).send().await?.json().await?;
    Ok(client)
}

#[tokio::main]
pub async fn show(
    user: String,
    api_key: String,
    title_as_url: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!(
        "{}{}{}{}",
        "https://api.omg.lol/address/", user, "/pastebin/", title_as_url
    );
    let client: serde_json::Value = reqwest::Client::new()
        .get(url)
        .bearer_auth(api_key)
        .send()
        .await?
        .json()
        .await?;
    Ok(client)
}

#[tokio::main]
pub async fn list(
    user: String,
    api_key: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}{}{}", "https://api.omg.lol/address/", user, "/pastebin");
    let client: serde_json::Value = reqwest::Client::new()
        .get(url)
        .bearer_auth(api_key)
        .send()
        .await?
        .json()
        .await?;
    Ok(client)
}
