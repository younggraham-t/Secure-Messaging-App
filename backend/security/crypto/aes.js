const crypto = require("crypto");

function encryptAES(text, key, iv) {
  const cipher = crypto.createCipheriv("aes-256-cbc", key, iv);

  const encrypted = Buffer.concat([
    cipher.update(text, "utf8"),
    cipher.final()
  ]);

  return encrypted.toString("base64");
}

function decryptAES(ciphertext, key, iv) {
  const buffer = Buffer.from(ciphertext, "base64");

  const decipher = crypto.createDecipheriv("aes-256-cbc", key, iv);

  const decrypted = Buffer.concat([
    decipher.update(buffer),
    decipher.final()
  ]);

  return decrypted.toString("utf8");
}

module.exports = { encryptAES, decryptAES };