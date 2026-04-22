use aes::Aes256;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::{Engine as _, engine::general_purpose};

type Aes256CbcEnc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;

/// Mirror of encryptAES from aes.js
pub fn encrypt_aes(text: &str, key: &[u8], iv: &[u8]) -> String {
    // Buffer must be a multiple of the block size (16)
    // PKCS7 padding adds between 1 and 16 bytes.
    let mut buf = vec![0u8; text.len() + 16]; 
    buf[..text.len()].copy_from_slice(text.as_bytes());
    
    let ct = Aes256CbcEnc::new(key.into(), iv.into())
        .encrypt_padded_mut::<cbc::cipher::block_padding::Pkcs7>(&mut buf, text.len())
        .expect("AES-CBC encryption failed");
        
    general_purpose::STANDARD.encode(ct)
}

/// Mirror of decryptAES from aes.js
pub fn decrypt_aes(ciphertext: &str, key: &[u8], iv: &[u8]) -> String {
    let mut buf = general_purpose::STANDARD.decode(ciphertext).expect("Base64 decode failed");
    
    let pt = Aes256CbcDec::new(key.into(), iv.into())
        .decrypt_padded_mut::<cbc::cipher::block_padding::Pkcs7>(&mut buf)
        .expect("AES-CBC decryption failed");
        
    String::from_utf8(pt.to_vec()).expect("Invalid UTF-8 sequence")
}
