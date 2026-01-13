# Video Streaming Server - WebTransport PoC

Proof of Concept for a video streaming server using WebTransport (QUIC/HTTP3).

## 🚀 Quick Start

### Development Mode
```bash
# Install dependencies
cargo build

# Run with auto-generated self-signed certificate
cargo run

# Open in Chrome
http://127.0.0.1:8080
```

### Production Mode

#### Set environment variable
```bash
export PRODUCTION=true
```

#### Generate ECDSA certificates (WebTransport compatible)
```bash
# Generate ECDSA private key (prime256v1 curve)
openssl ecparam -genkey -name prime256v1 -out key.pem

# Generate certificate valid for 10 days (WebTransport requires < 14 days)
openssl req -new -x509 -key key.pem -out cert.pem -days 10 \
  -subj '/CN=localhost' \
  -addext "subjectAltName = DNS:localhost,IP:127.0.0.1,IP:::1"

# Verify the certificate uses ECDSA (should show "id-ecPublicKey")
openssl x509 -in cert.pem -text -noout | grep -A2 "Public Key Algorithm"

# Check file permissions
ls -lh *.pem
```

#### Run the server
```bash
# Run with generated certificates
PRODUCTION=true cargo run
```

## ⚠️ WebTransport Certificate Requirements

WebTransport has strict certificate requirements:
- ✅ **ECDSA** algorithm (RSA not supported)
- ✅ Validity **< 14 days**
- ✅ Subject Alternative Name (SAN) must be present

## 🔐 Certificate Modes

| Mode | Command | Usage |
|------|---------|-------|
| **Self-signed** | `cargo run` | ✅ Development (works immediately in browser) |
| **Custom .pem files** | Place cert.pem + key.pem | ⚠️ Testing (may require browser acceptance) |
| **Production** | `PRODUCTION=true` | ✅ Production (certificates required) |

## 📦 Project Structure
```
src/
├── main.rs           # Entry point with certificate management
├── init.rs           # Server configuration
├── connection.rs     # WebTransport connection handling
├── http_server.rs    # HTTP server for web interface
└── html_content.rs   # Client interface (HTML/CSS/JS)
```

## 🎬 Supported Commands

- `START_VIDEO` - Start video streaming
- `GET_METADATA` - Retrieve video metadata
- Custom commands via web interface

## 🛠️ Technologies

- **Rust** with Tokio (async runtime)
- **WebTransport** (wtransport 0.6.1)
- **Axum** (HTTP server framework)
- **QUIC/HTTP3** for low-latency transport

## 🧪 Testing

### Development Testing
1. Start server: `cargo run`
2. Open Chrome: `http://127.0.0.1:8080`
3. Click "Connect"
4. Test commands (START_VIDEO, GET_METADATA, etc.)

### Production Testing
1. Generate certificates (see Production Mode above)
2. Start server: `PRODUCTION=true cargo run`
3. Test in Chrome as above

## 🐛 Troubleshooting

### Connection fails with "Opening handshake failed"
- Clear Chrome cache: `Cmd + Shift + Delete`
- Completely quit and restart Chrome: `Cmd + Q`
- Ensure certificates are ECDSA (not RSA)

### "Failed to parse private key"
- Ensure key is in PKCS8 format (starts with `-----BEGIN PRIVATE KEY-----`)
- Convert if needed: `openssl pkcs8 -topk8 -nocrypt -in old_key.pem -out key.pem`

### Certificate not trusted
- In development mode, use `cargo run` (self-signed works automatically)
- In production, use Let's Encrypt or properly signed certificates

## 📝 Production Deployment

For real production deployment with Let's Encrypt:
```bash
# Install certbot
sudo apt install certbot

# Generate ECDSA certificate
sudo certbot certonly --standalone \
  -d your-domain.com \
  --key-type ecdsa \
  --elliptic-curve secp256r1

# Run with Let's Encrypt certificates
PRODUCTION=true \
CERT_PATH=/etc/letsencrypt/live/your-domain.com/fullchain.pem \
KEY_PATH=/etc/letsencrypt/live/your-domain.com/privkey.pem \
cargo run --release
```

## 📄 License

Apache 2.0 - See [LICENSE](LICENSE) for details

## 👤 Author

Nils MARTIN - Video Streaming PoC for Interview