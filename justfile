set shell := ["zsh", "-cu"]

@dev:
    bun run tauri dev

@build-macos:
    bun tauri build --target aarch64-apple-darwin --bundles app
