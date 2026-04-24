use yew::prelude::*;
use crate::Language;

#[function_component(Cv)]
pub fn cv(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    html! {
        <div class="container" style="padding: 100px 24px;">
            <h2 style="font-size: 2.5rem; font-weight: 900; margin-bottom: 50px;">{ if is_en { "EVOLUTION_LOG" } else { "工程歷程" } }</h2>
            <div style="border-left: 2px solid var(--primary); padding-left: 30px; position: relative;">
                <div style="margin-bottom: 40px;">
                    <div style="color: var(--primary); font-family: 'JetBrains Mono', monospace; font-size: 0.8rem;">{ "2026 - PRESENT" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "MSI Innovation Center - Project Partner" } else { "微星科技創新前瞻中心 - 合作夥伴" } }</h3>
                    <p style="color: #888;">{ if is_en { "Developing infrastructure for AI Digital Twins." } else { "負責 AI 數位孿生基礎設施建置。" } }</p>
                </div>
                <div>
                    <div style="color: var(--primary); font-family: 'JetBrains Mono', monospace; font-size: 0.8rem;">{ "2026 FALL" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "UC Cincinnati - MS in Mechanical Engineering" } else { "美國辛辛那提大學 - 機械工程碩士" } }</h3>
                    <p style="color: #888;">{ if is_en { "Dual Degree Program." } else { "雙聯學位計劃。" } }</p>
                </div>
            </div>
        </div>
    }
}
