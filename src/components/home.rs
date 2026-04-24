use yew::prelude::*;
use crate::Language;

#[derive(Properties, PartialEq)]
pub struct PageProps { pub lang: Language }

#[function_component(Home)]
pub fn home(props: &PageProps) -> Html {
    let is_en = props.lang == Language::En;
    html! {
        <div class="reveal" style="min-height: 90vh; display: flex; flex-direction: column; justify-content: center; align-items: center; text-align: center; padding: 0 20px;">
            <h1 style="font-size: clamp(3rem, 12vw, 8rem); font-weight: 900; letter-spacing: -4px; line-height: 0.9; margin-bottom: 20px; background: linear-gradient(to bottom, #fff 30%, #444 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">
                { "LIN\nSHU-JEN" }
            </h1>
            <div style="font-family: 'JetBrains Mono', monospace; color: var(--accent); border: 1px solid var(--accent); padding: 8px 20px; border-radius: 4px; font-size: clamp(0.8rem, 2vw, 1.1rem); margin-bottom: 30px;">
                { if is_en { "> INITIALIZING SYSTEMS_ARCHITECT.EXE" } else { "> 系統架構師核心啟動中_" } }
            </div>
            <p style="color: #aaa; max-width: 600px; line-height: 1.8; font-size: 1.1rem; font-weight: 400;">
                { if is_en { "Bridging Digital Twins and Embodied AI. GTC 2026 Exhibitor. Engineering the future with Rust & Wasm." } 
                  else { "橫跨數位孿生與具身智能。NVIDIA GTC 2026 參展人。用 Rust 與 WebAssembly 打造極致系統。" } }
            </p>
        </div>
    }
}
