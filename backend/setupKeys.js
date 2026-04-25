const fs = require("fs");
const path = require("path");
const { generateKeyPair } = require("./security/crypto/rsa");

// Resolves to the absolute path of the root /keys directory
const keysPath = path.resolve(__dirname, "../keys");

// Create folder if not exists
if (!fs.existsSync(keysPath)) {
  fs.mkdirSync(keysPath, { recursive: true });
}

const bob = generateKeyPair();
fs.writeFileSync(path.join(keysPath, "bob_public.pem"), bob.publicKey);
fs.writeFileSync(path.join(keysPath, "bob_private.pem"), bob.privateKey);

const alice = generateKeyPair();
fs.writeFileSync(path.join(keysPath, "alice_public.pem"), alice.publicKey);
fs.writeFileSync(path.join(keysPath, "alice_private.pem"), alice.privateKey);

const mallory = generateKeyPair();
fs.writeFileSync(path.join(keysPath, "mallory_public.pem"), mallory.publicKey);
fs.writeFileSync(path.join(keysPath, "mallory_private.pem"), mallory.privateKey);

console.log(`\nSuccess: Keys generated/updated in: ${keysPath}\n`);
