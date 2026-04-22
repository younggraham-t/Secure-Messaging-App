const fs = require("fs");
const path = require("path");
const { generateKeyPair } = require("./security/crypto/rsa");

const dir = path.join(__dirname, "security", "keys");

// Create folder if not exists
if (!fs.existsSync(dir)) {
  fs.mkdirSync(dir, { recursive: true });
}

const { publicKey, privateKey } = generateKeyPair();

fs.writeFileSync(path.join(dir, "bob_public.pem"), publicKey);
fs.writeFileSync(path.join(dir, "bob_private.pem"), privateKey);

const { alicePublicKey, alicePrivateKey } = generateKeyPair();

fs.writeFileSync(path.join(dir, "alice_public.pem"), alicePublicKey);
fs.writeFileSync(path.join(dir, "alice_private.pem"),alicePrivateKey);

const { malloryPublicKey, malloryPrivateKey } = generateKeyPair();

fs.writeFileSync(path.join(dir, "mallory_public.pem"), malloryPublicKey);
fs.writeFileSync(path.join(dir, "mallory_private.pem"),malloryPrivateKey);

console.log("Keys generated in security/keys/");
