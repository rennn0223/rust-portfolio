use yew::prelude::*;
use crate::Language;

#[derive(Properties, PartialEq)]
pub struct PageProps { pub lang: Language }

#[function_component(Home)]
pub fn home(props: &PageProps) -> Html {
    let is_en = props.lang == Language::En;
    html! {
        <div style="min-height: 100vh; display: flex; flex-direction: column; justify-content: center; align-items: center; text-align: center; padding: 120px 20px 40px;">
            <h1 style="font-size: clamp(2.8rem, 10vw, 6.5rem); font-weight: 900; letter-spacing: -2px; background: linear-gradient(to bottom, #fff, #666); -webkit-background-clip: text; -webkit-text-fill-color: transparent; line-height: 1.1; margin-bottom: 15px;">
                { "LIN SHU-JEN" }
            </h1>
            <div style="font-family: 'JetBrains Mono', monospace; background: rgba(0,0,0,0.5); padding: 10px 20px; border-radius: 8px; border: 1px solid rgba(255,255,255,0.1); color: #10b981; margin-bottom: 25px; font-size: clamp(0.85rem, 3vw, 1.2rem);">
                { if is_en { "> NVIDIA GTC 2026 Exhibitor & AI Architect_" } else { "> NVIDIA GTC 2026 參展人與 AI 架構師_" } }
            </div>
            <p style="color: #888; max-width: 650px; line-height: 1.8; font-size: 1.05rem;">
                { if is_en { "Pioneering Digital Twins & Embodied AI. Collaborative partner with MSI Innovation Center and NVIDIA GTC 2026 Project Contributor. Incoming Dual MS @ UC." } 
                  else { "專注於數位孿生與具身智能。微星科技創新前瞻中心合作夥伴，NVIDIA GTC 2026 專案核心貢獻者。即將前往美國辛辛那提大學攻讀雙聯碩士。" } }
            </p>
        </div>
    }
}
