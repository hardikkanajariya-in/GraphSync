# Contributing to GraphSync

Thank you for helping improve GraphSync. This project is a community-maintained desktop sync client built with Tauri, Rust, and React.

## Ways to contribute

- Report bugs
- Suggest features
- Improve documentation
- Fix issues or implement enhancements
- Review pull requests

## Development setup

1. Install prerequisites from the [README](README.md)
2. Clone the repository
3. Install dependencies:

```bash
pnpm install
```

4. Start the app:

```bash
pnpm tauri dev
```

## Pull request guidelines

1. Open an issue first for large changes
2. Keep pull requests focused and small when possible
3. Match existing code style and project structure
4. Do not commit secrets, tokens, or local config files
5. Update documentation when behavior changes

## Commit messages

Use clear, descriptive commit messages:

- `fix: handle websocket reconnect backoff`
- `feat: add settings field for device name`
- `docs: clarify release install steps`

## Testing

Before opening a pull request:

- GitHub Actions runs a Rust compile check on Ubuntu (`cargo check`) — no website or desktop bundle is built in CI.
- Desktop release builds (Windows + macOS `.dmg`) run only in the Release workflow when a `v*` tag is pushed.
- Website changes are validated by Vercel preview/production deploys.

If you are changing the desktop app locally:

```bash
cargo check --manifest-path src-tauri/Cargo.toml
```

CI also runs on pull requests automatically.

## Releases

Maintainers create releases by pushing a version tag:

```bash
git tag v1.0.0
git push origin v1.0.0
```

GitHub Actions builds Windows and macOS installers and publishes them to GitHub Releases.

You can also run the release workflow manually from the Actions tab.

## Code of conduct

Please read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before participating.

## Security

If you find a security issue, do not open a public issue. See [SECURITY.md](SECURITY.md).
