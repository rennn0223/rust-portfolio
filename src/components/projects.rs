use yew::prelude::*;
use crate::Language;

#[function_component(Projects)]
pub fn projects(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    let card_class = "glass";
    let card_style = "padding: 40px; display: flex; flex-direction: column; gap: 20px;";
    let tag_style = "font-family: 'JetBrains Mono', monospace; font-size: 0.7rem; color: var(--primary); border: 1px solid var(--primary); padding: 2px 8px; border-radius: 4px; width: fit-content;";

    html! {
        <div class="container" style="padding: 120px 24px;">
            <h2 style="font-size: 3.5rem; font-weight: 900; margin-bottom: 60px; letter-spacing: -2px;">
                { if is_en { "TECH_SOLUTIONS" } else { "核心技術開發" } }
            </h2>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(350px, 1fr)); gap: 40px;">
                /* 專案 1: NVIDIA GTC 2026 */
                <div class={card_class} style={card_style}>
                    <div style="display: flex; gap: 10px;">
                        <span style={tag_style}>{ "OMNIVERSE" }</span>
                        <span style={tag_style}>{ "ALPAMAYO" }</span>
                        <span style={tag_style}>{ "DIGITAL_TWIN" }</span>
                    </div>
                    <h3 style="font-size: 1.8rem; font-weight: 900;">{ if is_en { "Campus Inspection & Ackermann DT" } else { "校園巡檢與阿克曼小車數位孿生" } }</h3>
                    <p style="color: #aaa; line-height: 1.8;">
                        { if is_en { 
                            "Exhibited at NVIDIA GTC 2026 (DGX Spark booth). Implemented a comprehensive Digital Twin for Ackermann-steering vehicles. Deeply researched NVIDIA Alpamayo applications for synchronized physical-virtual environments. Recognized with a signature from Jensen Huang." 
                        } else { 
                            "於 NVIDIA GTC 2026 微星 DGX Spark 攤位展出。實現阿克曼轉向小車的完整數位孿生系統，並深入研究 NVIDIA Alpamayo 應用於虛實整合同步，獲得執行長黃仁勳親筆簽名肯定。" 
                        } }
                    </p>
                </div>

                /* 專案 2: MGX & NemoClaw */
                <div class={card_class} style={card_style}>
                    <div style="display: flex; gap: 10px;">
                        <span style={tag_style}>{ "MGX_SERVER" }</span>
                        <span style={tag_style}>{ "NEMOCLAW" }</span>
                        <span style={tag_style}>{ "LLM" }</span>
                    </div>
                    <h3 style="font-size: 1.8rem; font-weight: 900;">{ if is_en { "MGX Infrastructure & NemoClaw AI" } else { "MGX 基礎設施與 NemoClaw AI 整合" } }</h3>
                    <p style="color: #aaa; line-height: 1.8;">
                        { if is_en { 
                            "Managing MGX Server upgrades and environment architecture. Deploying NemoClaw on DGX Spark to lower the entry barrier for NVIDIA Omniverse through natural language interaction and AI-driven automation." 
                        } else { 
                            "負責 MGX Server 升級與環境建置。在 DGX Spark 上部署 NemoClaw，試圖透過 AI 自然語言互動解決新手上手 Omniverse 的技術門檻問題。" 
                        } }
                    </p>
                </div>

                /* 專案 3: Autonomous Systems */
                <div class={card_class} style={card_style}>
                    <div style="display: flex; gap: 10px;">
                        <span style={tag_style}>{ "ROS2" }</span>
                        <span style={tag_style}>{ "LiDAR" }</span>
                        <span style={tag_style}>{ "SENSORS" }</span>
                    </div>
                    <h3 style="font-size: 1.8rem; font-weight: 900;">{ if is_en { "Autonomous Navigation & Sensor Fusion" } else { "自動導航與感測器融合" } }</h3>
                    <p style="color: #aaa; line-height: 1.8;">
                        { if is_en { 
                            "Developed ROS2-based control systems. Integrated and calibrated multi-sensor arrays including LiDAR, IMU, and Cameras for Ackermann-type vehicles." 
                        } else { 
                            "開發基於 ROS2 的控制系統。整合並校準包含 LiDAR、IMU 與攝影機在內的多感測器陣列，應用於阿克曼架構車輛之定位與導航。" 
                        } }
                    </p>
                </div>
            </div>
        </div>
    }
}
