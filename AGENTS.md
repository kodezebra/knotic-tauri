# Knotic — AI Reference

## Project
Tauri v2 desktop markdown note-taking app with Svelte 5 frontend and Rust backend.

## Commands

| Command | Description |
|---------|-------------|
| `bun dev` | Start Vite dev server (frontend only) |
| `bunx tauri dev` | Start Tauri app in dev mode |
| `bun build` | Build frontend for production |
| `bunx tauri build` | Build production Tauri app (frontend + Rust) |
| `bun run check` | Type-check Svelte files |
| `bun run lint` | Check formatting with Prettier |
| `bun run format` | Format all files |

## Release process

1. Update version in `package.json` and `src-tauri/tauri.conf.json`
2. Commit changes
3. Create and push a tag: `git tag v0.1.0 && git push origin v0.1.0`
4. GitHub Actions builds Linux (.deb, .AppImage) and Windows (.msi) and creates a GitHub Release

## CI
GitHub Actions workflow at `.github/workflows/build.yml`. Builds on push to `main`, PRs to `main`, and tag pushes `v*`.

## Package manager
Bun is the primary package manager (`bun.lock`).

## Rust MSRV
1.77.2 (defined in `src-tauri/Cargo.toml`)
