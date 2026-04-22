const fs = require("fs");
const path = require("path");

const KEYS_DIR = path.join(__dirname, "../keys");

function getPublicKey(user) {
	const userKeyPath = user + "_public.pem"
  const keyPath = path.join(KEYS_DIR, userKeyPath);
  if (!fs.existsSync(keyPath)) {
    throw new Error("[CA] " + userKeyPath + " not found. Run: node setupKeys.js first.");
  }
  return fs.readFileSync(keyPath, "utf8");
}

function getPrivateKey() {
  const keyPath = path.join(KEYS_DIR, "bob_private.pem");
  if (!fs.existsSync(keyPath)) {
    throw new Error("[CA] bob_private.pem not found. Run: node setupKeys.js first.");
  }
  return fs.readFileSync(keyPath, "utf8");
}

module.exports = { getPublicKey, getPrivateKey };
