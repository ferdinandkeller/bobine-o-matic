set shell := ["zsh", "-c"]

@dev:
    bun run tauri dev

# @back:
#     cd src-tauri/src-python && uv run main.py --file-path="/Users/ferdinandkeller/Downloads/Direct Media EKE 2.xlsx" --security-coeff=3.08 --window-size=7 --delivery-duration=30 --order-frequency=7 --download=false

# @build-back:
#     rm -rf src-tauri/src-python/dist/main
#     # cd src-tauri/src-python && uv run pyinstaller --onedir --exclude-module=pyinstaller main.py
#     cd src-tauri/src-python && uv run pyinstaller --onefile --exclude-module=pyinstaller main.py
#     # mv src-tauri/src-python/dist/main/main src-tauri/src-python/dist/main/main-aarch64-apple-darwin
#     mv src-tauri/src-python/dist/main src-tauri/src-python/dist/main-aarch64-apple-darwin

# @back-builded:
#     ./src-tauri/src-python/dist/main/main-aarch64-apple-darwin --file-path="/Users/ferdinandkeller/Downloads/Direct Media EKE.xlsx" --security-coeff=3.08 --window-size=7 --delivery-duration=30 --order-frequency=7 --download=false

# @build: build-back build-fast

# @build-fast:
#     bun tauri build --target aarch64-apple-darwin --bundles app
