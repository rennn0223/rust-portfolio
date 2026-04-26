use yew::prelude::*;
use crate::Language;

#[derive(Properties, PartialEq)]
pub struct PageProps { pub lang: Language }

#[function_component(Home)]
pub fn home(props: &PageProps) -> Html {
    let is_en = props.lang == Language::En;
    html! {
        <div class="page-transition" style="min-height: 85vh; display: flex; flex-direction: column; justify-content: center; align-items: center; text-align: center; padding: 0 24px;">
            <div class="typing-container">
                <div class="typing" style="font-family: 'JetBrains Mono', monospace; color: var(--accent); margin-bottom: 20px; font-size: clamp(0.7rem, 3vw, 0.9rem);">
                    { if is_en { "> INITIALIZING_SYSTEMS_ARCHITECT_CORE_" } else { "> 系統架構師核心已載入_" } }
                </div>
            </div>
            <h1 style="font-size: clamp(3rem, 11vw, 8.5rem); font-weight: 900; letter-spacing: -4px; line-height: 0.9; margin-bottom: 30px; background: linear-gradient(to bottom, #fff 40%, #555 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">
                { "LIN,\nSHU-JEN" }
            </h1>
            <p style="color: #999; max-width: 650px; line-height: 1.8; font-size: 1.1rem;">
                { if is_en { "Partnering with MSI Innovation Center. NVIDIA GTC 2026 Exhibitor. Specializing in Digital Twins & Embodied AI Infrastructure." } 
                  else { "微星科技創新前瞻中心夥伴。NVIDIA GTC 2026 參展人。專注於數位孿生、具身智能與高效能系統架構。" } }
            </p>
            <div style="margin-top: 40px; display: flex; gap: 15px; flex-wrap: wrap; justify-content: center;">
                <span style="font-family: 'JetBrains Mono', monospace; font-size: 0.8rem; border: 1px solid rgba(255,255,255,0.2); padding: 4px 10px; border-radius: 20px;">{ "RUST" }</span>
                <span style="font-family: 'JetBrains Mono', monospace; font-size: 0.8rem; border: 1px solid rgba(255,255,255,0.2); padding: 4px 10px; border-radius: 20px;">{ "WASM" }</span>
                <span style="font-family: 'JetBrains Mono', monospace; font-size: 0.8rem; border: 1px solid rgba(255,255,255,0.2); padding: 4px 10px; border-radius: 20px;">{ "OMNIVERSE" }</span>
            </div>
        </div>
    }
}
