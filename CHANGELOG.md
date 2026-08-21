# Changelog

All notable changes to GraphSync are documented here.

## 1.0.0 — 2026-08-21

First public release.

### Desktop client

- Tauri 2 shell with system tray, background sync, and minimal setup UI
- Rust sync engine with filesystem watching, SHA-256 hashing, and local state persistence
- WebRTC peer-to-peer file transfer with API-mediated signaling
- Conflict-safe delete handling and deterministic conflict copies
- Windows and macOS release builds via GitHub Actions

### Website

- Product landing page and readable documentation at `/docs`
- Vercel deployment from the repository root

### Notes

- `webrtc` remains on the 0.13 line until the peer manager is migrated to the 0.20 handler API
- Dependabot is configured to ignore automatic `webrtc >= 0.14` bumps until that migration lands

## 0.1.0

Initial development preview.
