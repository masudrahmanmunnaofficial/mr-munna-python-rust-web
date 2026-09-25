# MR.Munna Adventure Journey · Python + Rust Web

A separate website project built with Rust WebAssembly, Python build tools, and responsive HTML/CSS. The website is published through GitHub Pages at `https://masudrahmanmunnaofficial.github.io/mr-munna-python-rust-web/`.

## Build

`python3 -m unittest discover -s tools` validates static data. Install the Rust WASM target and Trunk, then run `trunk serve` locally or `trunk build --release --public-url /mr-munna-python-rust-web/` for GitHub Pages. GitHub Actions runs these checks and deploys `dist/`.

## Data

Browser records currently use localStorage. Each browser has its own data; clearing browser storage removes entries. Use **Backup & settings → Download backup** often. Backups are JSON. Large media attachments and offline calculated prayer times are not yet implemented in this web version. The app makes no server account or password claim; this GitHub Pages site and its source are public. Sexual Health and Offline Vault are absent.
