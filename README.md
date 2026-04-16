# Krause Method Matrix Generator (WASM Port)

This repository contains the Rust implementation of the Krause and Fulkerson matrix generation algorithms, compiled to WebAssembly (WASM) for rapid, client-side execution in the browser.

## Prerequisites

To compile and run this project locally, you will need the following tools installed on your system:

1. **Rust and Cargo**: The standard Rust toolchain.
2. **wasm-pack**: The build tool for Rust-generated WebAssembly.
3. **A Local Web Server**: WebAssembly files cannot be loaded over the `file://` protocol due to browser security restrictions. You will need a simple HTTP server (like Python's `http.server`, Node's `http-server`, or the VS Code Live Server extension).

## Build Instructions

1. `git clone https://github.com/Muhammad-Sawaiz-Karim/krauss-method-site`
2. `cd krauss-method-site`
3. `wasm-pack build --target web`
4. `python -m http.server 8000` or use VSCode Live Server
5. `http://localhost:8000` in your browser