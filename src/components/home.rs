use yew::prelude::*;
use crate::Language;

#[derive(Properties, PartialEq)]
pub struct PageProps { pub lang: Language }

#[function_component(Home)]
pub fn home(props: &PageProps) -> Html {
    let is_en = props.lang == Language::En;
    html! {
        <div style="min-height: 85vh; display: flex; flex-direction: column; justify-content: center; align-items: center; text-align: center;">
            <h1 style="font-size: clamp(3rem, 12vw, 8.5rem); font-weight: 900; letter-spacing: -4px; line-height: 0.85; margin-bottom: 25px; background: linear-gradient(to bottom, #fff 40%, #555 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">
                { "SHU-JEN\nLIN" }
            </h1>
            <div style="font-family: 'JetBrains Mono', monospace; border: 1px solid var(--primary); color: var(--primary); padding: 8px 20px; border-radius: 4px; font-size: 0.9rem; margin-bottom: 30px;">
                { if is_en { "> SYSTEMS ARCHITECT | GTC 2026 EXHIBITOR" } else { "> 系統架構師 | NVIDIA GTC 2026 參展人" } }
            </div>
            <p style="color: #888; max-width: 600px; line-height: 1.8; font-size: 1.1rem;">
                { if is_en { "Specializing in Digital Twins & Embodied AI. Engineering the future with Rust & Wasm." } 
                  else { "專注於數位孿生與具身智能。以 Rust 與 WebAssembly 驅動下一代系統開發。" } }
            </p>
        </div>
    }
}
