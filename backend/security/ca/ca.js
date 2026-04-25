const fs = require("fs");
const path = require("path");

// Resolves to the absolute path of the root /keys directory
const keysPath = path.resolve(__dirname, "../../../keys");

function getPublicKey(user) {
  const keyPath = path.join(keysPath, `${user.toLowerCase()}_public.pem`);
  if (!fs.existsSync(keyPath)) {
    throw new Error(`[CA] Public key for ${user} not found at ${keyPath}`);
  }
  return fs.readFileSync(keyPath, "utf8");
}

function getPrivateKey(user) {
  const keyPath = path.join(keysPath, `${user.toLowerCase()}_private.pem`);
  if (!fs.existsSync(keyPath)) {
    throw new Error(`[CA] Private key for ${user} not found at ${keyPath}`);
  }
  return fs.readFileSync(keyPath, "utf8");
}

module.exports = { getPublicKey, getPrivateKey };
