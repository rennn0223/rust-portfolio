use yew::prelude::*;
use crate::Language;

#[function_component(Cv)]
pub fn cv(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    let timeline_item = "margin-bottom: 50px; position: relative; border-left: 2px solid rgba(59, 130, 246, 0.3); padding-left: 35px;";
    let dot = "position: absolute; left: -9px; top: 8px; width: 16px; height: 16px; background: var(--primary); border-radius: 50%; box-shadow: 0 0 10px var(--primary); border: 3px solid var(--bg);";
    let btn_style = "display: inline-block; margin-top: 12px; color: var(--accent); border: 1px solid var(--accent); padding: 5px 12px; border-radius: 4px; text-decoration: none; font-size: 0.75rem; font-family: 'JetBrains Mono'; transition: 0.3s;";

    html! {
        <div class="container" style="padding: 120px 24px;">
            <h2 style="font-size: 3.5rem; font-weight: 900; margin-bottom: 60px;">{ if is_en { "EXPERIENCE_LOG" } else { "精確任務日誌" } }</h2>
            
            <div style="max-width: 850px;">
                /* 2026.03.30 - Now */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style="color: var(--primary); font-family: 'JetBrains Mono';">{ "2026.03.30 - PRESENT" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "MGX Server & NemoClaw Research" } else { "MGX Server 升級與 NemoClaw 應用" } }</h3>
                    <p style="color: #bbb;">{ if is_en { "Handling hardware infrastructure and LLM-driven automation on DGX Spark." } else { "負責 MGX 伺服器硬體基礎建設與 DGX Spark 上的 AI 自動化部署。" } }</p>
                </div>

                /* 2026.03.05 - DLI Certificate */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style="color: var(--primary); font-family: 'JetBrains Mono';">{ "2026.03.05" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "NVIDIA DLI Certificate" } else { "NVIDIA DLI 專業認證" } }</h3>
                    <p style="color: #bbb;">{ if is_en { "Passed 'Isaac for Accelerated Robotics'." } else { "通過 Isaac 機器人加速運算課程。" } }</p>
                    <a href="你的證書連結" target="_blank" class="hover-btn" style={btn_style}>{ "> VERIFY_CREDENTIAL" }</a>
                </div>

                /* 2026.01 - GTC & MSI HQ */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style="color: var(--primary); font-family: 'JetBrains Mono';">{ "2026.01 - 2026.03" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "GTC Exhibition & MSI Collaboration" } else { "GTC 參展與微星總部實作" } }</h3>
                    <p style="color: #bbb;">{ if is_en { "Worked with Manager Shi-Zhe Hung & RD Yan-Cheng Lai at MSI HQ. Visited UC Berkeley BAIR Lab." } else { "於微星總部與洪士哲經理及賴彥成工程師協同開發。展後拜訪 UC Berkeley AI 自駕中心。" } }</p>
                    <a href="你的照片集連結" target="_blank" style={btn_style}>{ "> VIEW_EVENT_GALLERY" }</a>
                </div>

                /* 2025.10.30 - DJI */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style="color: var(--primary); font-family: 'JetBrains Mono';">{ "2025.10.30" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "DJI Drone Training" } else { "大疆無人機培訓課程" } }</h3>
                    <p style="color: #bbb;">{ if is_en { "Completed professional UAV operation program." } else { "完成大疆專業無人機操作培訓。" } }</p>
                    <a href="你的證明連結" target="_blank" style={btn_style}>{ "> VIEW_PROOF" }</a>
                </div>
            </div>
        </div>
    }
}
