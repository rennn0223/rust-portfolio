# LIN, SHU-JEN | Systems Architect Portfolio

A high-performance, WebAssembly-powered personal portfolio built with Rust and Yew. 
Focusing on Digital Twins, Embodied AI Infrastructure, and NVIDIA Omniverse integrations.

## 🛠 Tech Stack
- **Rust**: Core logic and state management.
- **Yew**: Component-based frontend framework.
- **Trunk**: WASM web application bundler.
- **CSS3**: Custom animations, glassmorphism, and responsive RWD design.

## 🚀 How to Run Locally
1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Add WebAssembly target: `rustup target add wasm32-unknown-unknown`
3. Install Trunk: `cargo install trunk`
4. Run development server: `trunk serve`
5. Open browser at `http://localhost:8080`

## 🌍 Deployment
This project is automatically deployed to GitHub Pages via GitHub Actions upon pushing to the `main` branch.
