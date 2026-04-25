use crate::crypto::{aes_cbc, hmac, rsa};
use rand::{thread_rng, RngCore};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct SessionInit {
    pub session_key: Vec<u8>,
    pub encrypted_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessagePayload {
    pub iv: String,
    pub ciphertext: String,
    pub hmac: String,
}



pub fn initiate_session(public_key_pem: &str) -> SessionInit {
    let mut session_key = [0u8; 32];
    thread_rng().fill_bytes(&mut session_key);

    //using the public key to encrypt the AES session key so it can be sent to the recipient
    let encrypted_key_bytes = rsa::encrypt_with_public_key(&session_key, public_key_pem);
    let encrypted_key = general_purpose::STANDARD.encode(encrypted_key_bytes);

    SessionInit {
        session_key: session_key.to_vec(),
        encrypted_key,
    }
}

pub fn receive_session(enc_key_base64: &str, private_key_pem: &str) -> Vec<u8> {
    let enc_key_bytes = general_purpose::STANDARD.decode(enc_key_base64).expect("Invalid base64 in session key");
    rsa::decrypt_with_private_key(&enc_key_bytes, private_key_pem)
}

pub fn encrypt_message(message: &str, session_key: Vec<u8>) -> MessagePayload {

    let mut iv_bytes = [0u8; 16];
    //provides good entropy for crypto use
    thread_rng().fill_bytes(&mut iv_bytes);
    //encode iv as base64
    let iv = general_purpose::STANDARD.encode(iv_bytes);

    log::info!("{}", session_key.len());
    let key: [u8; 32] = session_key.try_into().expect("Session key length mismatch");
    // session_key is the AES key generated in initiate_session
    let ciphertext = aes_cbc::encrypt_aes(message, &key, &iv_bytes);

    //append iv (base64) to ciphertext (base64)
    let hmac_input = format!("{}{}", ciphertext, iv);
    let hmac = hmac::generate_hmac(hmac_input.as_bytes(), &key);

    MessagePayload {
        iv,
        ciphertext,
        hmac,
    }
}

pub fn decrypt_message(payload: &MessagePayload, session_key: &[u8]) -> Result<String, String> {
    // Node: verifyHMAC(payload.ciphertext + payload.iv, sessionKey, payload.hmac)
    let hmac_input = format!("{}{}", payload.ciphertext, payload.iv);
    
    let valid = hmac::verify_hmac(
        hmac_input.as_bytes(),
        session_key,
        &payload.hmac
    );

    if !valid {
        return Err("Integrity compromised: HMAC verification failed".to_string());
    }

    let iv_bytes = general_purpose::STANDARD.decode(&payload.iv).expect("Invalid base64 in IV");
    
    return Ok(aes_cbc::decrypt_aes(&payload.ciphertext, session_key, &iv_bytes))
}
