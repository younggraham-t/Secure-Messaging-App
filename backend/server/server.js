const express = require('express');
const http = require('http');
const { WebSocketServer } = require('ws');
const cors = require('cors');
const CA = require("../security/ca/ca");
const { receiveSession, initiateSession, encryptMessage, decryptMessage } = require("../security/services/session");
const chalk = require("chalk");

const magenta = chalk.hex('#FF00FF');
const green = chalk.hex('#008000');
const red = chalk.hex('#FF0000');
const lightGreen = chalk.hex("#39e75f");

const app = express();
app.use(cors());
app.use(express.json());

const server = http.createServer(app);
const wss = new WebSocketServer({ server });

const port = 3000;

const connectedUsers = new Map();
const interceptedSessions = new Map();

app.get('/ca/public-key/:user', (req, res) => {
    try {
        const data = CA.getPublicKey(req.params.user);
        res.json({ data });
    } catch (e) {
        res.status(404).json({ error: e.message });
    }
});

app.get('/user/public-key/:user', (req, res) => {
    console.log(magenta('[MALLORY]') + red(` Intercepting public key request for ${req.params.user}`));
    console.log(magenta('[MALLORY]') + red(` Sending own public key`));
    const malloryKey = CA.getPublicKey("mallory");
    res.json({ data: malloryKey });
});

// --- WebSocket Relay Logic ---
wss.on('connection', (ws) => {
    let currentUsername = "";

    ws.on('message', (message) => {
        try {
            const data = JSON.parse(message);
            const type = data.type;
			// console.log(data)

            if (type === 'register') {
                currentUsername = data.username.toLowerCase();
                connectedUsers.set(currentUsername, ws);
                console.log(green('[SERVER]') + ` ${data.username} connected.`);
            }

            else if (type === 'session_start') {
                const { to, encrypted_key } = data;
                const recipient = to.toLowerCase();
                console.log(magenta('[MALLORY]') + ` Handshake intercepted: ${currentUsername} -> ${recipient}`);

                try {
                    const malloryPrivate = CA.getPrivateKey("mallory");
                    const decryptedSessionKey = receiveSession(encrypted_key, malloryPrivate);

                    console.log(magenta('[MALLORY]') + red(` ATTACK SUCCESSFUL! Decrypted session key from ${currentUsername}: ${decryptedSessionKey.toString('hex')}`));
                    
                    const bobPublicKey = CA.getPublicKey(recipient);
                    console.log(magenta('[MALLORY]') + ` Initiating Session with ${recipient}`);
                    const malloryToBob = initiateSession(bobPublicKey);

                    interceptedSessions.set(currentUsername, {
                        keyAlice: decryptedSessionKey,
                        keyBob: malloryToBob.sessionKey
                    });

                    const bobWs = connectedUsers.get(recipient);
                    if (bobWs) {
                        bobWs.send(JSON.stringify({
                            type: "session_receive",
                            from: currentUsername,
                            encrypted_key: malloryToBob.encrypted_key
                        }));
                    }
                } catch (e) {
                    console.log(magenta('[MALLORY]') + lightGreen(` Handshake is SECURE. Cannot insert into session.`));
                    const bobWs = connectedUsers.get(recipient);
                    if (bobWs) {
                        bobWs.send(JSON.stringify({
                            type: "session_receive",
                            from: currentUsername,
                            encrypted_key: encrypted_key
                        }));
                    }
                }
            }

            else if (type === 'message') {
                const { to, payload } = data;
                const recipient = to.toLowerCase();
                const session = interceptedSessions.get(currentUsername);
				// console.log(session)

                console.log(magenta('[MALLORY]') + ` Trying to decrypt incoming message: "${payload.ciphertext}"`);
                if (session) {
                    try {
                        const plaintext = decryptMessage(payload, session.keyAlice);
                        console.log(magenta('[MALLORY]') + red(` Decrypt Success, Message: "${plaintext}"`));

						const modifiedPlaintext = plaintext.replace("me $", "Mallory $")
						
                        
                        const newPayload = encryptMessage(modifiedPlaintext, session.keyBob);
                        const bobWs = connectedUsers.get(recipient);
                        if (bobWs) {
                            bobWs.send(JSON.stringify({
                                type: "message_receive",
                                from: currentUsername,
                                payload: newPayload
                            }));
                        }
                    } catch (e) {
                        console.log(magenta('[MALLORY]') + ` Relay failed: ${e.message}`);
                    }
                } else {
                    console.log(magenta('[MALLORY]') + lightGreen(` Cannot decrypt, no MITM session`))
					console.log(green('[SERVER]') + ` Forwarding encrypted blob ${currentUsername} -> ${recipient}`);
                    const bobWs = connectedUsers.get(recipient);
                    if (bobWs) {
                        bobWs.send(JSON.stringify({
                            type: "message_receive",
                            from: currentUsername,
                            payload: payload
                        }));
                    }
                }
            }
        } catch (err) {
            console.error("[SERVER] Invalid message format:", err);
        }
    });

    ws.on('close', () => {
        if (currentUsername) connectedUsers.delete(currentUsername);
        console.log(green('[SERVER]') + ` ${currentUsername} disconnected.`);
    });
});

server.listen(port, () => {
    console.log(`\n==============================================`);
    console.log(green('[SERVER]') + ` RUNNING ON PORT ${port}`);
    console.log(`==============================================\n`);
	
    console.log(magenta('[MALLORY]') + ` Listening to all Conections`);
});
