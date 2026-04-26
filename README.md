This is the term project for ITIS6200. It is written in Rust and Javascript. 





To run:
Ensure you have Rust and Trunk, and Node installed

https://rust-lang.org/tools/install/ 

https://trunk-rs.github.io/trunk/ 

https://nodejs.org/en/download 

1. Open the backend directory to install dependencies
```
(cd backend && npm i)
```
2. run the runner from the root directory
```
node run.js
```
This starts the /backend/server/server.js and runs `trunk serve` to start the frontend. After a few seconds it will open 2 browser tabs. 

For all cases: Open each browser and choose the "signed in" user (Bob on one and Alice on the other) then choose the recipient as the other person. 
# Success Case
When "Sucure Mode" is activated the application will fetch the recipient's public key from the "trusted" CA source. In both cases the system will send a "session start" message to the server with an encrypted session key. In secure mode Mallory will attempt to decrypt the session key and be unsuccessful. Then the server will forward the encrypted key to the recipient. The client will then send a payload encrypted with the session key to the server which will forward it to the other client. At this stage Mallory will again try to decrypt and be unsuccessful.

# Failure Case
In "insecure" mode the client will fetch the recipient's public key from the "untrusted" source. In insecure mode Mallory will intercept the public key request and return her own key. The browser will then start a session with what it believes to be the correct person but who is actually Mallory. Mallory will start sessions with the sender and the recipient and will be able to decrypt any messages that are sent using the session key. NOTE: Mallory currently only looks for the session key from the original sender so messages sent by the original recipient will not be sent in the current state of the application.
