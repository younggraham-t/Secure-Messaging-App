const net = require("net");

const {
  receiveSession,
  decryptMessage
} = require("../security/services/session");

const CA = require("../security/ca/ca");

const privateKey = CA.getPrivateKey();

const client = net.createConnection({ port: 5000 });

let sessionKey;

client.on("connect", () => {
  client.write(JSON.stringify({
    type: "init_bob",
    publicKey: CA.getPublicKey()
  }));
});

client.on("data", (data) => {
  const msg = JSON.parse(data.toString());

  // Secure mode
  if (msg.type === "secure_session") {
    sessionKey = receiveSession(msg.encrypted_key, privateKey);
    console.log("[Bob] Secure session established\n");
  }

  // MITM mode
  if (msg.type === "session_bob") {
    sessionKey = receiveSession(msg.encrypted_key, privateKey);
    console.log("[Bob] Session established (MITM scenario)\n");
  }

  if (msg.type === "message_to_bob") {
    const text = decryptMessage(msg.payload, sessionKey);
    console.log("\n[Bob receives]:", text);
  }
});

client.on("error", (err) => {
  console.log("[Bob error]:", err.message);
});