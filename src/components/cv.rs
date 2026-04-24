use yew::prelude::*;
use crate::Language;

#[function_component(Cv)]
pub fn cv(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    let timeline_item = "margin-bottom: 60px; position: relative; border-left: 2px solid rgba(59, 130, 246, 0.3); padding-left: 35px;";
    let dot = "position: absolute; left: -7px; top: 8px; width: 12px; height: 12px; background: var(--primary); border-radius: 50%; box-shadow: 0 0 10px var(--primary);";

    html! {
        <div class="container" style="padding: 120px 24px;">
            <h2 style="font-size: 3.5rem; font-weight: 900; margin-bottom: 60px;">
                { if is_en { "PROFESSIONAL_TIMELINE" } else { "職涯與學術歷程" } }
            </h2>
            
            <div style="max-width: 800px;">
                /* MSI 歷程 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style="font-family: 'JetBrains Mono', monospace; color: var(--primary); font-size: 0.9rem;">{ "2026 - PRESENT" }</div>
                    <h3 style="font-size: 1.6rem; margin: 10px 0;">{ if is_en { "MSI Innovation Center" } else { "微星科技創新前瞻中心" } }</h3>
                    <div style="color: var(--accent); font-weight: bold; margin-bottom: 15px;">{ if is_en { "External Project Partner / Architect" } else { "外部專案合作夥伴 / 架構師" } }</div>
                    <ul style="color: #bbb; line-height: 1.8; padding-left: 20px;">
                        <li>{ if is_en { "Architecting Digital Twin pipelines using NVIDIA Omniverse Enterprise." } else { "使用 NVIDIA Omniverse Enterprise 建構數位孿生流水線。" } }</li>
                        <li>{ if is_en { "Implementing Wasm-driven edge computing modules for MSI server hardware." } else { "為微星伺服器硬體實現 Wasm 驅動的邊緣運算模組。" } }</li>
                        <li>{ if is_en { "Leading cross-disciplinary technical integration for high-performance AI systems." } else { "領導高效能 AI 系統的跨學科技術整合。" } }</li>
                    </ul>
                </div>

                /* 辛辛那提大學 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style="font-family: 'JetBrains Mono', monospace; color: var(--primary); font-size: 0.9rem;">{ "2026 FALL" }</div>
                    <h3 style="font-size: 1.6rem; margin: 10px 0;">{ if is_en { "University of Cincinnati (UC)" } else { "美國辛辛那提大學" } }</h3>
                    <div style="color: var(--accent); font-weight: bold; margin-bottom: 15px;">{ if is_en { "Master of Science in Mechanical Engineering" } else { "機械工程碩士" } }</div>
                    <p style="color: #bbb; line-height: 1.8;">
                        { if is_en { 
                            "Selected for the prestigious Dual Degree Program. Focusing research on the intersection of Robotics, Digital Twin synchronization, and Embodied Intelligence." 
                        } else { 
                            "入選雙聯學位計劃。研究方向專注於機器人學、數位孿生同步協定與具身智能之結合。" 
                        } }
                    </p>
                </div>

                /* 本科/其他 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style="font-family: 'JetBrains Mono', monospace; color: #666; font-size: 0.9rem;">{ "2022 - 2026" }</div>
                    <h3 style="font-size: 1.6rem; margin: 10px 0;">{ if is_en { "Home University (Taiwan)" } else { "台灣原就讀大學" } }</h3>
                    <p style="color: #666; line-height: 1.8;">
                        { if is_en { "B.S. in Mechanical Engineering. Foundation in systems control and automation." } else { "機械工程學士。奠定系統控制與自動化基礎。" } }
                    </p>
                </div>
            </div>
        </div>
    }
}
