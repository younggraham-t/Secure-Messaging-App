const crypto = require("crypto");
const { encryptAES, decryptAES } = require("../crypto/aes");
const { encryptWithPublicKey, decryptWithPrivateKey } = require("../crypto/rsa");
const { generateHMAC, verifyHMAC } = require("../crypto/hmac");

function initiateSession(publicKey) {
  const sessionKey = crypto.randomBytes(32);

  const encryptedKey = encryptWithPublicKey(sessionKey, publicKey);

  return {
    sessionKey,
    encrypted_key: encryptedKey.toString("base64")
  };
}

function receiveSession(encKey, privateKey) {
  return decryptWithPrivateKey(
    Buffer.from(encKey, "base64"),
    privateKey
  );
}

function encryptMessage(message, sessionKey) {
  const iv = crypto.randomBytes(16);

  const ciphertext = encryptAES(message, sessionKey, iv);

  const hmac = generateHMAC(
    ciphertext + iv.toString("base64"),
    sessionKey
  );

  return {
    iv: iv.toString("base64"),
    ciphertext,
    hmac
  };
}

function decryptMessage(payload, sessionKey) {
  const valid = verifyHMAC(
    payload.ciphertext + payload.iv, // payload.iv is already base64
    sessionKey,
    payload.hmac
  );

  if (!valid) throw new Error("Integrity compromised");

  return decryptAES(
    payload.ciphertext,
    sessionKey,
    Buffer.from(payload.iv, "base64")
  );
}

module.exports = {
  initiateSession,
  receiveSession,
  encryptMessage,
  decryptMessage
};