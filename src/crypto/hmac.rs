use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{Engine as _, engine::general_purpose};

type HmacSha256 = Hmac<Sha256>;

/// Mirror of generateHMAC from hmac.js
pub fn generate_hmac(data: &[u8], key: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    let result = mac.finalize();
    general_purpose::STANDARD.encode(result.into_bytes())
}

/// Mirror of verifyHMAC from hmac.js
pub fn verify_hmac(data: &[u8], key: &[u8], received: &str) -> bool {
    let received_bytes = match general_purpose::STANDARD.decode(received) {
        Ok(b) => b,
        Err(_) => return false,
    };
    
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    
    // verify_slice provides constant-time comparison like timingSafeEqual
    mac.verify_slice(&received_bytes).is_ok()
}
