# Release process

GraphSync desktop releases are built automatically by GitHub Actions.

The docs site is deployed separately by Vercel on push to the connected branch. GitHub Actions does not build the website.

## Automatic release on tag push

1. Update the version in:
   - `package.json`
   - `website/package.json`
   - `src-tauri/tauri.conf.json`
   - `src-tauri/Cargo.toml`
2. Commit the version bump and update `CHANGELOG.md`
3. Create and push a tag:

```bash
git tag v1.0.0
git push origin v1.0.0
```

The [Release workflow](.github/workflows/release.yml) builds:

- Windows x64 installer (`.msi` / `.exe`)
- macOS Apple Silicon `.dmg`
- macOS Intel `.dmg`

Artifacts are attached to the GitHub Release for the tag.

## Manual release

Maintainers can also run the workflow from the GitHub Actions tab using **Run workflow**.

Provide a version such as `1.0.0`. The workflow creates the `v1.0.0` tag if it does not already exist.

## Notes

- Releases are unsigned community builds by default
- Code signing can be added later with GitHub Actions secrets for Apple and Windows certificates
- The workflow also uploads build artifacts to the Actions run for debugging
