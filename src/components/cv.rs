use yew::prelude::*;
use crate::Language;

#[function_component(Cv)]
pub fn cv(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    html! {
        <div class="container" style="padding: 100px 24px;">
            <h2 style="font-size: 3rem; font-weight: 900; margin-bottom: 60px;">{ if is_en { "CHRONOLOGY" } else { "工程歷程" } }</h2>
            <div style="border-left: 1px solid rgba(255,255,255,0.1); padding-left: 40px;">
                <div style="margin-bottom: 60px; position: relative;">
                    <div style="position: absolute; left: -45px; top: 5px; width: 10px; height: 10px; background: var(--primary); border-radius: 50%;"></div>
                    <h3 style="color: var(--primary); font-family: 'JetBrains Mono', monospace;">{ "2026 - PRESENT" }</h3>
                    <h4 style="font-size: 1.4rem; margin: 10px 0;">{ if is_en { "MSI Innovation Center | Project Partner" } else { "微星科技創新前瞻中心 | 專案夥伴" } }</h4>
                    <ul style="color: #aaa; line-height: 1.8; padding-left: 20px;">
                        <li>{ if is_en { "Developing the next-gen AI inspection infrastructure." } else { "負責研發下一代 AI 巡檢基礎設施。" } }</li>
                        <li>{ if is_en { "Integrating NVIDIA Omniverse with localized enterprise solutions." } else { "將 NVIDIA Omniverse 與在地企業解決方案進行整合。" } }</li>
                    </ul>
                </div>
                <div style="margin-bottom: 60px; position: relative;">
                    <div style="position: absolute; left: -45px; top: 5px; width: 10px; height: 10px; background: #555; border-radius: 50%;"></div>
                    <h3 style="color: #888; font-family: 'JetBrains Mono', monospace;">{ "2026 FALL" }</h3>
                    <h4 style="font-size: 1.4rem; margin: 10px 0;">{ if is_en { "University of Cincinnati | MS in Mechanical Engineering" } else { "美國辛辛那提大學 | 機械工程碩士" } }</h4>
                    <p style="color: #888;">{ if is_en { "Research focusing on Digital Twin synchronization protocols." } else { "專注於數位孿生同步協定之研究。" } }</p>
                </div>
            </div>
        </div>
    }
}
