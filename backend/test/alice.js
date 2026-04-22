const net = require("net");
const readline = require("readline");

const {
  initiateSession,
  receiveSession,
  encryptMessage
} = require("../security/services/session");

const { generateKeyPair } = require("../security/crypto/rsa");
const CA = require("../security/ca/ca");

// TOGGLE FROM CLI
const mode = process.argv[2];

if (!mode) {
  console.log("Usage: node alice.js [secure | insecure]");
  process.exit();
}

const SECURE_MODE = mode === "secure";

// Alice MUST have her own key pair
const alice = generateKeyPair();

const client = net.createConnection({ port: 5000 });

let sessionKey;
let inputStarted = false;

client.on("connect", () => {

  if (SECURE_MODE) {
    console.log("[SECURE MODE]");
    console.log("[CA] Fetching Bob's public key from CA");

    const bobKey = CA.getPublicKey();

    const session = initiateSession(bobKey);
    sessionKey = session.sessionKey;

    client.write(JSON.stringify({
      type: "secure_session",
      encrypted_key: session.encrypted_key
    }));
  }

  else {
    console.log("[INSECURE MODE]");
    console.log("[WARNING] Using unverified key from network");

    // SEND ALICE PUBLIC KEY
    client.write(JSON.stringify({
      type: "init_alice",
      publicKey: alice.publicKey
    }));
  }
});

function startInput() {
  if (inputStarted) return;
  inputStarted = true;

  console.log("Alice ready. Type message:\n");

  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
  });

  rl.on("line", (input) => {
    const payload = encryptMessage(input, sessionKey);

    client.write(JSON.stringify({
      type: "message_from_alice",
      payload
    }));
  });
}

client.on("data", (data) => {
  const msg = JSON.parse(data.toString());

  // MITM MODE (FIXED)
  if (msg.type === "session_alice") {
    sessionKey = receiveSession(msg.encrypted_key, alice.privateKey);
    startInput();
  }

  // SECURE MODE
  if (msg.type === "ready") {
    startInput();
  }
});

client.on("error", (err) => {
  console.log("[Alice error]:", err.message);
});