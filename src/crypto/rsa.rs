use rsa::{RsaPublicKey, RsaPrivateKey, Oaep, pkcs1::DecodeRsaPublicKey, pkcs1::DecodeRsaPrivateKey, pkcs1::EncodeRsaPrivateKey, pkcs1::EncodeRsaPublicKey};
use rand::thread_rng;
use sha1::Sha1; // Node's default for OAEP unless specified


pub fn generate_key_pair() -> (String, String) {
    let mut rng = thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
    let pub_key = priv_key.to_public_key();

    let priv_pem = priv_key.to_pkcs1_pem(Default::default()).expect("Failed to encode private key");
    let pub_pem = pub_key.to_pkcs1_pem(Default::default()).expect("Failed to encode public key");

    (pub_pem.to_string(), priv_pem.to_string())
}

pub fn encrypt_with_public_key(data: &[u8], public_key_pem: &str) -> Vec<u8> {
    let pub_key = RsaPublicKey::from_pkcs1_pem(public_key_pem).expect("Invalid RSA Public Key PEM");
    let mut rng = thread_rng();
    
    // Node defaults to SHA1 for OAEP padding if not specified
    pub_key.encrypt(&mut rng, Oaep::new::<Sha1>(), data)
        .expect("RSA Encryption failed")
}

pub fn decrypt_with_private_key(data: &[u8], private_key_pem: &str) -> Vec<u8> {
    let priv_key = RsaPrivateKey::from_pkcs1_pem(private_key_pem).expect("Invalid RSA Private Key PEM");
    
    priv_key.decrypt(Oaep::new::<Sha1>(), data)
        .expect("RSA Decryption failed")
}
