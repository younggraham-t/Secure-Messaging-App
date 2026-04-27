const { spawn } = require('child_process');
const path = require('path');

console.log("==========================================");
console.log("   Launching Secure Messaging Simulation  ");
console.log("==========================================\n");

// Start the Frontend (Trunk) first
const frontend = spawn('trunk', ['serve'], {
    shell: true,
    stdio: 'inherit'
});

let backend;

// Wait 5 seconds for Trunk to initialize, then start Backend
setTimeout(() => {
    console.log("\n[STAGING] Starting Mallory Terminal...");
    backend = spawn('node', ['server/server.js'], {
        cwd: path.join(__dirname, 'backend'),
        shell: true,
        stdio: 'inherit'
    });
}, 5000);

// Automatically open browsers to the ui
setTimeout(() => {
    const url = 'http://127.0.0.1:8080';
    const start = process.platform === 'darwin' ? 'open' : process.platform === 'win32' ? 'start' : 'xdg-open';
    
    console.log(`\n[UI] Opening 2 browser tabs for Alice and Bob...\n`);
    spawn(start, [url], { shell: true });
    spawn(start, [url], { shell: true });
}, 5000);

// Handle Cleanup
const cleanup = () => {
    console.log("\n[SHUTDOWN] Stopping all servers...");
    if (backend) backend.kill();
    frontend.kill();
    process.exit();
};

process.on('SIGINT', cleanup);
process.on('SIGTERM', cleanup);
