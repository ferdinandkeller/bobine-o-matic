set shell := ["zsh", "-c"]

@dev:
    bun run tauri dev

@back:
    cd src-python && uv run main.py

@build-back:
    cd src-python && uv run pyinstaller --onefile main.py
