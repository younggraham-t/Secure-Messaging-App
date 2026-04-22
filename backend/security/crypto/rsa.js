const crypto = require("crypto");

function generateKeyPair() {
  return crypto.generateKeyPairSync("rsa", {
    modulusLength: 2048,
    publicKeyEncoding: { type: "pkcs1", format: "pem" },
    privateKeyEncoding: { type: "pkcs1", format: "pem" }
  });
}

function encryptWithPublicKey(data, publicKey) {
  return crypto.publicEncrypt(
    { key: publicKey, padding: crypto.constants.RSA_PKCS1_OAEP_PADDING },
    Buffer.isBuffer(data) ? data : Buffer.from(data)
  );
}

function decryptWithPrivateKey(data, privateKey) {
  return crypto.privateDecrypt(
    { key: privateKey, padding: crypto.constants.RSA_PKCS1_OAEP_PADDING },
    Buffer.isBuffer(data) ? data : Buffer.from(data)
  );
}

module.exports = { generateKeyPair, encryptWithPublicKey, decryptWithPrivateKey };