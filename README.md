

<div align="center">

# Hades

**A Discord webhook forwarder designed to protect your webhooks**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-v1.0.0-brightgreen.svg)](https://github.com/rescore/hades)

*Built in Rust • Inspired by [Hyperion](https://github.com/Rescore09/Hyperion/)*

</div>


## Features

| Feature                | Description                                                           |
| ---------------------- | --------------------------------------------------------------------- |
| **Message Forwarding** | Forward JSON messages to a predefined Discord webhook asynchronously  |
| **Rate Limiting**      | Per-client IP rate limiting with configurable limits and time windows |
| **Authentication**     | Secure `x-admin` header authentication to prevent unauthorized access |
| **Async Performance**  | Built with `tokio` + `reqwest` for fast, non-blocking forwarding      |
| **Structured Logging** | Logs successful sends, rate-limited attempts, and unauthorized access |

## Getting Started

### Prerequisites

```bash
# Required
Rust 1.70+
Cargo
Internet access
```

### Installation

```bash
git clone https://github.com/rescore/hades.git
cd hades
```

### Quick Start

```bash
# Build the project
cargo build --release

# Run with default configuration
cargo run

# Or run the binary directly
./target/release/hades
```

> ⚠️ By default, Hades runs on `127.0.0.1:3030`.
> If you are going to attempt to use this productionally you need to tinker with the code.

## API Usage

### Forward a message

```bash
curl -X POST http://127.0.0.1:3030/forward \
  -H "Content-Type: application/json" \
  -H "x-admin: supersecretkey123" \
  -d '{
    "content": "Hello from Hades!"
  }'
```

* Only requests with the correct `x-admin` key are accepted.
* Rate limiting is enforced per IP (default: 5 requests per 60 seconds).

## Configuration

Currently, Hades uses **hardcoded configuration** inside `main.rs`:

```rust
let state = AppState::new(
    "https://discord.com/api/webhooks/XXXXX/YYYYY", // target webhook
    "supersecretkey123",                             // admin key
    5,                                               // rate limit
    60,                                              // window in seconds
);
```

You can change the target webhook, admin key, rate limit, and window by editing the code.

## Performance

* **Async processing** with `tokio` runtime
* **Concurrent webhook sends** with `tokio::spawn`
* **Lightweight in-memory rate limiting**

---

## Developers note:
<b><i>This project isn't meant to be taken seriously and was only created to allow me to learn Rust through hard work. If you see any errors or inconsistencies, please note them so I can work on fixing them.</i></b>


<div align="center">
<B>❌ Built with ❤️</B><br>
<B>✅ Built with Rust</B>

[Report Bug](https://github.com/rescore/hades/issues) • [Request Feature](https://github.com/rescore/hades/issues)

</div>

