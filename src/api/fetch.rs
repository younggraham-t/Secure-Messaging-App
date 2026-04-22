use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PublicKeyResponse {
    pub key: String
}

pub async fn fetch_public_key(user: &str, endpoint: &str) -> Result<String, String> {
    let url = format!("{}/{}", endpoint, user);

    let res = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    
    if !res.ok() {
        return Err(format!("Server returned status: {}", res.status()));
    }

    let data: PublicKeyResponse = res
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    // Return the raw PEM string directly
    Ok(data.key)
}
