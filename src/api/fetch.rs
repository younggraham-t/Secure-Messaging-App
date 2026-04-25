use gloo_net::http::Request;
use serde::Deserialize;

use crate::crypto::session::MessagePayload;

#[derive(Deserialize)]
pub struct Response {
    pub data: String
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

    let data: Response = res
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    // Return the raw PEM string directly
    Ok(data.data)
}

pub async fn post_message(payload: MessagePayload) -> Result<String, String> {
    
    let url = "/api/message".to_string();
    let res = Request::post(&url)
        .json(&payload)
        .expect("Failed to serialize payload")
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if !res.ok() {
        return Err(format!("Server returned status: {}", res.status()));
    }
    
    let data: Response = res
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(data.data)
        
}

// pub fn get_messages(user: &str) {
//
// }
