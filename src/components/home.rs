use yew::prelude::*;
use crate::Language;

#[derive(Properties, PartialEq)]
pub struct PageProps { pub lang: Language }

#[function_component(Home)]
pub fn home(props: &PageProps) -> Html {
    let is_en = props.lang == Language::En;
    html! {
        <div style="min-height: 85vh; display: flex; flex-direction: column; justify-content: center; align-items: center; text-align: center; padding: 0 24px;">
            <div style="font-family: 'JetBrains Mono', monospace; color: var(--accent); margin-bottom: 20px;" class="typing">
                { if is_en { "> INITIALIZING_SYSTEMS_ARCHITECT_CORE" } else { "> 系統架構師核心已載入" } }
            </div>
            <h1 style="font-size: clamp(3rem, 12vw, 8.5rem); font-weight: 900; letter-spacing: -4px; line-height: 0.85; margin-bottom: 30px; background: linear-gradient(to bottom, #fff, #555); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">
                { "SHU-JEN\nLIN" }
            </h1>
            <p style="color: #888; max-width: 700px; line-height: 1.8; font-size: 1.15rem;">
                { if is_en { "Partnering with MSI Innovation Center. NVIDIA GTC 2026 Exhibitor. Specializing in Digital Twins & Embodied AI Infrastructure." } 
                  else { "微星科技創新前瞻中心夥伴。NVIDIA GTC 2026 參展人。專注於數位孿生、具身智能與高階系統架構。" } }
            </p>
            <div style="margin-top: 40px; display: flex; gap: 20px;">
                <div style="color: var(--primary); font-family: 'JetBrains Mono', monospace; border-bottom: 1px solid var(--primary); padding: 5px;">{ "#RUST" }</div>
                <div style="color: var(--primary); font-family: 'JetBrains Mono', monospace; border-bottom: 1px solid var(--primary); padding: 5px;">{ "#WASM" }</div>
                <div style="color: var(--primary); font-family: 'JetBrains Mono', monospace; border-bottom: 1px solid var(--primary); padding: 5px;">{ "#AI" }</div>
            </div>
        </div>
    }
}
