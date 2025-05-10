set shell := ["zsh", "-c"]

@dev:
    bun run tauri dev

@back:
    cd src-tauri/src-python && uv run main.py --file-path="/Users/ferdinandkeller/Downloads/Direct Media EKE.xlsx" --security-coeff=3.08 --delivery-duration=14 --download=false

@back-builded:
    ./src-tauri/src-python/dist/main/main-aarch64-apple-darwin --file-path="/Users/ferdinandkeller/Downloads/ventes.xlsx" --security-coeff=3.08 --delivery-duration=14

@build-back:
    cd src-tauri/src-python && uv run pyinstaller --onedir --exclude-module=pyinstaller main.py
    mv src-tauri/src-python/dist/main/main src-tauri/src-python/dist/main/main-aarch64-apple-darwin
