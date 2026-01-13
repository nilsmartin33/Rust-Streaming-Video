pub const INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Video Streaming - WebTransport</title>
    <link rel="stylesheet" href="style.css">
    <script src="client.js"></script>
</head>
<body>
    <h1> Video Streaming - WebTransport Demo</h1>

    <div class="section">
        <h2>1. Establish Connection</h2>
        <div class="input-line">
            <label for="url">Server URL:</label>
            <input type="text" id="url" value="https://localhost:${WEBTRANSPORT_PORT}/">
            <button id="connect" onclick="connect()">Connect</button>
        </div>
    </div>

    <div class="section">
        <h2>2. Send Commands</h2>
        <button onclick="sendStartVideo()" id="btnStartVideo" disabled> START_VIDEO</button>
        <button onclick="sendGetMetadata()" id="btnMetadata" disabled> GET_METADATA</button>
        <button onclick="sendCustom()" id="btnCustom" disabled> Custom Command</button>
    </div>

    <div class="section">
        <h2>3. Event Log</h2>
        <ul id="event-log"></ul>
    </div>
</body>
</html>
"#;

pub const STYLE_CSS: &str = r#"
body {
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    max-width: 900px;
    margin: 0 auto;
    padding: 20px;
    background-color: #f5f5f5;
}

h1 {
    text-align: center;
    color: #333;
    margin-bottom: 30px;
}

h2 {
    color: #555;
    border-bottom: 2px solid #4CAF50;
    padding-bottom: 5px;
    font-size: 1.2em;
}

.section {
    background: white;
    padding: 20px;
    margin-bottom: 20px;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.input-line {
    display: flex;
    gap: 10px;
    align-items: center;
    margin-bottom: 15px;
}

.input-line label {
    font-weight: bold;
    min-width: 100px;
}

.input-line input[type="text"] {
    flex: 1;
    padding: 8px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-family: monospace;
}

button {
    padding: 10px 20px;
    margin: 5px;
    border: none;
    border-radius: 4px;
    background-color: #4CAF50;
    color: white;
    font-size: 14px;
    cursor: pointer;
    transition: background-color 0.3s;
}

button:hover:not(:disabled) {
    background-color: #45a049;
}

button:disabled {
    background-color: #cccccc;
    cursor: not-allowed;
}

#event-log {
    list-style: none;
    padding: 0;
    max-height: 400px;
    overflow-y: auto;
    background: #f9f9f9;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-family: 'Courier New', monospace;
    font-size: 12px;
}

#event-log li {
    padding: 8px 12px;
    border-bottom: 1px solid #eee;
}

#event-log li:last-child {
    border-bottom: none;
}

.log-info { color: #2196F3; }
.log-success { color: #4CAF50; font-weight: bold; }
.log-error { color: #f44336; font-weight: bold; }
.log-warning { color: #FF9800; }
"#;

pub const CLIENT_JS: &str = r#"
const CERT_HASH = new Uint8Array(${CERT_DIGEST});

let currentTransport = null;

function addLog(message, type = 'info') {
    const log = document.getElementById('event-log');
    const entry = document.createElement('li');
    entry.className = `log-${type}`;
    entry.textContent = `[${new Date().toLocaleTimeString()}] ${message}`;
    log.appendChild(entry);
    log.scrollTop = log.scrollHeight;
}

async function connect() {
    const url = document.getElementById('url').value;

    try {
        addLog('Connecting to ' + url, 'info');

        const transport = new WebTransport(url, {
            serverCertificateHashes: [{
                algorithm: 'sha-256',
                value: CERT_HASH.buffer
            }]
        });

        await transport.ready;
        addLog('Connected successfully!', 'success');

        currentTransport = transport;

        // Enable buttons
        document.getElementById('btnStartVideo').disabled = false;
        document.getElementById('btnMetadata').disabled = false;
        document.getElementById('btnCustom').disabled = false;
        document.getElementById('connect').disabled = true;

        // Handle closure
        transport.closed.then(() => {
            addLog('Connection closed normally', 'info');
        }).catch((error) => {
            addLog('Connection closed with error: ' + error, 'error');
        });

    } catch (error) {
        addLog('Connection failed: ' + error, 'error');
    }
}

async function sendStartVideo() {
    await sendCommand('START_VIDEO');
}

async function sendGetMetadata() {
    await sendCommand('GET_METADATA');
}

async function sendCustom() {
    const command = prompt('Enter custom command:', 'HELLO');
    if (command) {
        await sendCommand(command);
    }
}

async function sendCommand(command) {
    if (!currentTransport) {
        addLog('Not connected!', 'error');
        return;
    }

    try {
        addLog('Sending: ' + command, 'info');

        const stream = await currentTransport.createBidirectionalStream();
        const writer = stream.writable.getWriter();
        const reader = stream.readable.getReader();

        const encoder = new TextEncoder();
        await writer.write(encoder.encode(command));
        await writer.close();

        const { value, done } = await reader.read();
        if (!done && value) {
            const decoder = new TextDecoder();
            const response = decoder.decode(value);
            addLog('Response: ' + response, 'success');
        }

    } catch (error) {
        addLog('Error: ' + error, 'error');
    }
}
"#;