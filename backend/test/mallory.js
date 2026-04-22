const net = require("net");

const {
  initiateSession,
  encryptMessage,
  decryptMessage
} = require("../security/services/session");

let aliceSocket = null;
let bobSocket = null;

let keyAlice = null;
let keyBob = null;

const server = net.createServer((socket) => {

  socket.on("data", (data) => {
    const msg = JSON.parse(data.toString());

    // INSECURE / MITM MODE
    // Alice sends her public key unverified. Mallory intercepts and
    // establishes her OWN session with Alice using Alice's public key.
    if (msg.type === "init_alice") {
      aliceSocket = socket;

      console.log("\n[MITM MODE ACTIVE]");
      console.log("[Mallory] Intercepted Alice's init. Acting as server.");

      const session = initiateSession(msg.publicKey);
      keyAlice = session.sessionKey;

      socket.write(JSON.stringify({
        type: "session_alice",
        encrypted_key: session.encrypted_key
      }));
    }

    // Bob announces himself — Mallory establishes a separate session with Bob
    else if (msg.type === "init_bob") {
      bobSocket = socket;

      console.log("[Mallory] Connected to real Bob. Establishing session.");

      const session = initiateSession(msg.publicKey);
      keyBob = session.sessionKey;

      socket.write(JSON.stringify({
        type: "session_bob",
        encrypted_key: session.encrypted_key
      }));
    }

    // SECURE MODE
    // Alice encrypted the session key using Bob's CA-verified public key.
    // Mallory cannot decrypt this — she doesn't have Bob's private key.
    // She forwards it blindly and signals ready to Alice.
    else if (msg.type === "secure_session") {
      console.log("\n[SECURE MODE ACTIVE]");
      console.log("[Mallory] Cannot decrypt this session key. Forwarding blindly.");

      if (bobSocket) {
        bobSocket.write(JSON.stringify(msg));
      }

      socket.write(JSON.stringify({ type: "ready" }));
    }

    // MESSAGE RELAY
    else if (msg.type === "message_from_alice") {

      if (!bobSocket) {
        console.log("[!] Bob not connected yet.");
        return;
      }

      // Secure mode — keyAlice is null because Mallory never established
      // a session with Alice. She can only forward the ciphertext she cannot read.
      if (!keyAlice) {
        console.log("[Mallory] Forwarding encrypted blob (cannot read it).");
        bobSocket.write(JSON.stringify({
          type: "message_to_bob",
          payload: msg.payload
        }));
        return;
      }

      // MITM mode — Mallory has both session keys
      // She decrypts Alice's message, optionally modifies it,
      // then re-encrypts with Bob's session key and forwards.
      try {
        const original = decryptMessage(msg.payload, keyAlice);

        console.log("\n[Mallory intercepted]:", original);
        console.log("Modify message (press Enter to keep original): ");

        process.stdin.resume();
        process.stdin.setEncoding("utf8");

        process.stdin.once("data", (input) => {
          process.stdin.pause();

          const modified = input.trim() === "" ? original : input.trim();

          if (modified !== original) {
            console.log("[Mallory] Message modified to:", modified);
          } else {
            console.log("[Mallory] Forwarding original.");
          }

          const newPayload = encryptMessage(modified, keyBob);

          bobSocket.write(JSON.stringify({
            type: "message_to_bob",
            payload: newPayload
          }));
        });

      } catch (err) {
        console.log("[Mallory] Decrypt failed:", err.message);
      }
    }
  });

  socket.on("error", (err) => {
    console.log("[Mallory socket error]:", err.message);
  });
});

server.listen(5000, () => {
  console.log("Mallory (MITM proxy) running on port 5000...\n");
  console.log("Start Bob first, then Alice.\n");
});