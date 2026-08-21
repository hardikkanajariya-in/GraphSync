# GraphSync

[![CI](https://github.com/graphsync/graphsync/actions/workflows/ci.yml/badge.svg)](https://github.com/graphsync/graphsync/actions/workflows/ci.yml)
[![Release](https://github.com/graphsync/graphsync/actions/workflows/release.yml/badge.svg)](https://github.com/graphsync/graphsync/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Author & maintainer:** [Hardik Kanajariya](https://hardikkanajariya.in)

GraphSync is a lightweight cross-platform desktop folder synchronization client. It watches a local folder, publishes change metadata to a GraphSync API, and transfers file bytes directly between devices over encrypted WebRTC data channels.

Files remain on your devices. The API stores coordination metadata only.

## Product site & docs

The public landing page and reader-friendly documentation live in [`website/`](website/). Deploy that folder to Vercel for a hosted product site and explorable docs.

## Download

Prebuilt installers are published on [GitHub Releases](https://github.com/graphsync/graphsync/releases).

| Platform | Download |
| --- | --- |
| Windows x64 | `.msi` or `.exe` from the latest release |
| macOS Apple Silicon | `.dmg` built for `aarch64-apple-darwin` |
| macOS Intel | `.dmg` built for `x86_64-apple-darwin` |

Releases are created automatically when a version tag such as `v0.1.0` is pushed to GitHub.

If you are forking this repository, replace `graphsync/graphsync` in the README badges and release links with your GitHub organization and repository name.

## Architecture

```text
Local folder
    ↓ notify (debounced)
Rust sync engine
    ↓ metadata
GraphSync API  ←→  WebSocket notifications
    ↓ signaling
WebRTC data channel
    ↓ file bytes
Peer device
```

- **UI**: React + TypeScript (setup, status, settings only)
- **Engine**: Rust + Tokio (filesystem, sync, networking, tray)
- **Shell**: Tauri 2

See [docs/API_CONTRACT.md](docs/API_CONTRACT.md) for the server protocol.

## Prerequisites

- Node.js 20+
- pnpm 9+
- Rust stable (via [rustup](https://rustup.rs/))
- Platform dependencies for Tauri 2: https://tauri.app/start/prerequisites/

## Installation

```bash
pnpm install
```

## Development

Start the desktop app:

```bash
pnpm tauri dev
```

Frontend only:

```bash
pnpm dev
```

## Build

Production frontend bundle:

```bash
pnpm build
```

Desktop bundles (.exe on Windows, .app on macOS):

```bash
pnpm tauri build
```

Artifacts are written to `src-tauri/target/release/bundle/`.

## First run

1. Launch GraphSync
2. Choose a folder to sync
3. Enter your GraphSync API URL
4. Click **Start Sync**

The app saves configuration under the OS application data directory, registers the device, starts the Rust sync engine, hides the main window, and continues from the system tray.

Tray menu:

- Open GraphSync
- Pause Sync / Resume Sync
- Open Sync Folder
- Settings
- Quit

## Configuration

Stored at:

- Windows: `%APPDATA%\\GraphSync\\config.json`
- macOS: `~/Library/Application Support/GraphSync/config.json`
- Linux: `~/.local/share/GraphSync/config.json`

Example:

```json
{
  "api_url": "https://sync.example.com",
  "sync_folder": "C:\\Users\\User\\Documents\\Logseq\\MyGraph",
  "device_id": "uuid",
  "device_name": "Windows PC",
  "enabled": true,
  "paused": false,
  "setup_complete": true,
  "stun_servers": ["stun:stun.l.google.com:19302"],
  "turn_servers": []
}
```

STUN/TURN servers are configurable for production relay fallback.

## Sync protocol overview

1. Local filesystem events are debounced and hashed (SHA-256, streamed)
2. Metadata events are sent to the API
3. Other devices receive events via WebSocket/polling
4. Devices exchange WebRTC offers/answers/ICE candidates through the API
5. Missing files transfer peer-to-peer over an encrypted data channel
6. Conflicts create deterministic `filename (conflict - Device - YYYY-MM-DD).ext` copies

Deletions are synchronized unless a local unsynced modification would be destroyed.

## Security model

- API tokens stay in Rust; the frontend never receives secrets
- Only normalized relative paths are transmitted
- Path traversal (`..`) is rejected
- Writes are constrained to the configured sync directory
- Atomic replace is used for incoming files
- File contents and tokens are never logged

## Logging

Daily rotating logs:

```text
<app-data>/GraphSync/logs/graphsync.log
```

The Settings screen shows the log directory.

## Troubleshooting

- **Setup fails immediately**: verify the API URL and that the GraphSync API implements `docs/API_CONTRACT.md`
- **Sync stays degraded**: API unreachable; events are queued with retry/backoff
- **Peer transfer fails**: configure STUN/TURN, ensure both devices are online, and confirm signaling websocket connectivity
- **High CPU while editing**: normal; GraphSync debounces rapid saves (~750ms)

## Important limitation

This repository contains the **desktop client only**. A compatible GraphSync API server is required for authentication, event storage, websocket notifications, and WebRTC signaling. P2P transfers also require reachable STUN and, for restrictive networks, production TURN infrastructure.

The WebRTC implementation is real (`webrtc-rs`), but end-to-end sync against production networks has not been validated here without a live API and multiple online peers.

## Community

- [Authors](AUTHORS.md)
- [Contributing guide](CONTRIBUTING.md)
- [Code of conduct](CODE_OF_CONDUCT.md)
- [Security policy](SECURITY.md)
- [Release process](.github/RELEASE.md)
- [API contract](docs/API_CONTRACT.md)
- [Product website source](website/)

## License

GraphSync is released under the [MIT License](LICENSE).

Copyright © 2026 [Hardik Kanajariya](https://hardikkanajariya.in)
