use yew::prelude::*;
use crate::Language;

#[function_component(Cv)]
pub fn cv(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    let timeline_item = "margin-bottom: 50px; position: relative; border-left: 2px solid rgba(59, 130, 246, 0.3); padding-left: 35px;";
    let dot = "position: absolute; left: -7px; top: 8px; width: 12px; height: 12px; background: var(--primary); border-radius: 50%; box-shadow: 0 0 10px var(--primary);";

    html! {
        <div class="container" style="padding: 120px 24px;">
            <h2 style="font-size: 3.5rem; font-weight: 900; margin-bottom: 60px;">{ if is_en { "MISSION_LOG" } else { "任務日誌" } }</h2>
            
            <div style="max-width: 900px;">
                /* 2026 - Now */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style="color: var(--primary); font-family: 'JetBrains Mono', monospace;">{ "2026.03 - PRESENT" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "Infrastructure & Embodied AI" } else { "基礎設施與具身智能深化" } }</h3>
                    <ul style="color: #bbb; line-height: 1.8;">
                        <li>{ if is_en { "Managing MGX Server upgrades and environment deployment." } else { "處理 MGX Server 升級、環境建置及應用開發。" } }</li>
                        <li>{ if is_en { "Implementing NemoClaw on DGX Spark to optimize Omniverse user experience." } else { "在 DGX Spark 安裝 NemoClaw 以優化 Omniverse 操作門檻。" } }</li>
                        <li>{ if is_en { "Certified: NVIDIA DLI - Isaac for Accelerated Robotics (2026.03.05)." } else { "獲得 NVIDIA DLI：Isaac 機器人加速運算課程通過證書 (2026.03.05)。" } }</li>
                    </ul>
                </div>

                /* GTC 2026 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style="color: var(--primary); font-family: 'JetBrains Mono', monospace;">{ "2026.01 - 2026.03" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "NVIDIA GTC 2026 & MSI Collaboration" } else { "NVIDIA GTC 2026 與微星協同實作" } }</h3>
                    <p style="color: #bbb; line-height: 1.7;">
                        { if is_en { 
                            "Collaborated at MSI Headquarters (Zhonghe) with Senior Manager Shi-Zhe Hung and RD Yan-Cheng Lai. Co-developed the Omniverse Campus Inspection and Ackermann DT project. Invited to visit UC Berkeley BAIR Lab for autonomous driving discussions after GTC exhibition." 
                        } else { 
                            "進駐微星中和總部，與資深經理洪士哲及 RD 賴彥成協同開發 GTC 展出專案。於加州 GTC 展期後，受邀訪問 UC Berkeley BAIR (AI 自駕中心) 進行技術交流。" 
                        } }
                    </p>
                </div>

                /* 2025 Deep Learning */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style="color: var(--primary); font-family: 'JetBrains Mono', monospace;">{ "2025.04 - 2025.12" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "System Control & Lab Foundation" } else { "實驗室研究與系統控制基礎" } }</h3>
                    <ul style="color: #bbb; line-height: 1.8;">
                        <li>{ if is_en { "2025.06: Initiated NVIDIA Omniverse x MSI Innovation Center collaboration." } else { "2025.06: 開啟 NVIDIA Omniverse 與 MSI 創新前瞻中心合作案。" } }</li>
                        <li>{ if is_en { "2025.08: Mastered ROS2, LiDAR, IMU, and Camera integration for Ackermann vehicles." } else { "2025.08: 掌握 ROS2 控制、LiDAR、IMU 與攝影機等感測器融合技術。" } }</li>
                        <li>{ if is_en { "2025.10: Certified in DJI Drone Training program." } else { "2025.10: 通過大疆無人機培訓並獲得證明。" } }</li>
                        <li>{ if is_en { "2025.04: Lab entry: Focus on Golf Cart Control, Linux, MySQL, and Unity." } else { "2025.04: 進入實驗室學習高爾夫球車控制、Linux、MySQL 及 Unity。" } }</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
