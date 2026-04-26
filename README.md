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
