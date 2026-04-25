use yew::prelude::*;
use crate::Language;

#[function_component(Cv)]
pub fn cv(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    let timeline_item = "margin-bottom: 60px; position: relative; border-left: 2px solid rgba(59,130,246,0.3); padding-left: 35px;";
    let dot = "position: absolute; left: -9px; top: 8px; width: 16px; height: 16px; background: var(--primary); border-radius: 50%; box-shadow: 0 0 12px var(--primary); border: 3px solid #030303;";
    let btn_style = "display: inline-block; margin: 10px 10px 0 0; color: var(--accent); border: 1px solid rgba(16, 185, 129, 0.4); padding: 6px 14px; border-radius: 4px; text-decoration: none; font-size: 0.75rem; font-family: 'JetBrains Mono'; transition: 0.3s; background: rgba(16, 185, 129, 0.05);";
    let date_style = "color: var(--primary); font-family: 'JetBrains Mono'; font-weight: bold; font-size: 0.95rem; margin-bottom: 5px;";

    html! {
        <div class="container" style="padding: 100px 24px;">
            <div style="margin-bottom: 50px; display: flex; flex-wrap: wrap; gap: 15px;">
                <a href="https://www.linkedin.com/in/rennn223" target="_blank" style={btn_style}>{ "LINKEDIN ↗" }</a>
                <a href="https://certs.duolingo.com/tlegwwbno75h9itb" target="_blank" style={btn_style}>{ "DUOLINGO_ENGLISH_PRO ↗" }</a>
            </div>

            <h2 style="font-size: 3rem; font-weight: 900; margin-bottom: 60px;">{ if is_en { "TIMELINE" } else { "精確任務時間軸" } }</h2>
            
            <div style="max-width: 900px;">
                /* Phase 6 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style={date_style}>{ "2026.03.30 - PRESENT" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "MGX Infrastructure & NemoClaw Deployment" } else { "MGX 伺服器建置與 NemoClaw 部署" } }</h3>
                    <p style="color: #bbb; line-height: 1.8;">{ if is_en { "Handling MGX Server upgrades, environment setup, and deploying NemoClaw on DGX Spark to lower Omniverse learning curves." } else { "處理 MGX Server 硬體升級與環境建置，並於 DGX Spark 安裝 NemoClaw，試圖解決新手難以上手 Omniverse 的問題。" } }</p>
                    <div style="margin-top: 15px; display: flex; flex-wrap: wrap;">
                        <a href="https://learn.nvidia.com/certificates?id=AOuaSDlrRjSNIw37SgD1VQ" target="_blank" style={btn_style}>{ "ISAAC ROBOTICS CERT ↗" }</a>
                        <a href="https://learn.nvidia.com/certificates?id=1DHB-ztRROWGqdjyu6qqTQ" target="_blank" style={btn_style}>{ "OPENUSD STAGES CERT ↗" }</a>
                        <a href="https://learn.nvidia.com/certificates?id=EN5-FdNJT_KR9akW3bacrg" target="_blank" style={btn_style}>{ "JETSON AI CERT ↗" }</a>
                    </div>
                </div>

                /* Phase 5 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <img src="assets/GTC2026.jpeg" alt="GTC 2026" style="width: 100%; max-width: 600px; border-radius: 8px; margin: 15px 0; border: 1px solid rgba(255,255,255,0.1);" />
                    <div style={date_style}>{ "2026.03.08 - 2026.03.21" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "NVIDIA GTC Exhibition & BAIR Visit" } else { "NVIDIA GTC 參展與 UC Berkeley 訪談" } }</h3>
                    <p style="color: #bbb; line-height: 1.8;">
                        { if is_en { "Traveled to California for NVIDIA GTC 2026. Exhibited Campus Patrol Omniverse Project at MSI DGX Spark booth. Received Jensen Huang's signature. Later invited to UC Berkeley BAIR for technical discussions." } 
                          else { "前往美國加州參加 NVIDIA GTC 2026。於微星 DGX Spark 攤位展出校園巡檢及阿克曼小車數位孿生專案，獲得黃仁勳親簽。展後受邀拜訪 UC Berkeley BAIR (AI自駕中心) 參觀及討論。" } }
                    </p>
                </div>

                /* Phase 4.5 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style={date_style}>{ "2026.03.05" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "NVIDIA DLI Certification" } else { "NVIDIA DLI 專業認證" } }</h3>
                    <p style="color: #bbb; line-height: 1.8;">
                        { if is_en { "Obtained NVIDIA DLI: Isaac for Accelerated Robotics Certificate." } 
                          else { "在 NVIDIA DLI 獲得 NVIDIA Isaac for Accelerated Robotics 課程通過證書。" } }
                    </p>
                </div>

                /* Phase 4 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style={date_style}>{ "2026.01 - 2026.02 (Before Lunar New Year)" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "MSI HQ Resident Implementation" } else { "微星中和總部協同實作" } }</h3>
                    <p style="color: #bbb; line-height: 1.8;">
                        { if is_en { "Resident at MSI HQ (Zhonghe). Collaborated with Senior Manager Shi-Zhe Hung & RD Yan-Cheng Lai to develop GTC demo and research Alpamayo." } 
                          else { "前往新北中和微星公司，與創新前瞻中心資深經理洪士哲及 RD 賴彥成協同實作 GTC 展出的 Omniverse 校園巡檢系統，並共同研究 NVIDIA Alpamayo 應用。" } }
                    </p>
                </div>

                /* Phase 3 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style={date_style}>{ "2025.10.30" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "DJI Drone Training" } else { "大疆無人機培訓課程" } }</h3>
                    <p style="color: #bbb; line-height: 1.8;">
                        { if is_en { "Participated in DJI UAV training program and received official certification." } 
                          else { "參加大疆無人機培訓課程並獲得參加證明。" } }
                    </p>
                </div>

                /* Phase 2.5 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style={date_style}>{ "2025.08.30" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "ROS2 & Sensor Integration" } else { "ROS2 控制與感測器啟動" } }</h3>
                    <p style="color: #bbb; line-height: 1.8;">
                        { if is_en { "Learned ROS2 control systems. Activated and integrated sensors including LiDAR, IMU, Cameras, and Ackermann Car architectures." } 
                          else { "學習 ROS2 控制並開啟感測器整合，諸如 LiDAR, IMU, Camera, 以及 Ackermann Car 控制。" } }
                    </p>
                </div>

                /* Phase 2 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style={date_style}>{ "2025.06.30" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "NVIDIA x MSI Partnership Initiated" } else { "NVIDIA 與微星合作案啟動" } }</h3>
                    <p style="color: #bbb; line-height: 1.8;">
                        { if is_en { "Began collaboration project connecting NVIDIA Omniverse with MSI Innovation Center." } 
                          else { "開始接洽 NVIDIA Omniverse 與 MSI 創新前瞻中心的正式合作案。" } }
                    </p>
                </div>

                /* Phase 1 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style={date_style}>{ "2025.04.30" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "Lab Entry & Foundation" } else { "進入實驗室與底層架構學習" } }</h3>
                    <p style="color: #bbb; line-height: 1.8;">
                        { if is_en { "Joined the lab. Began studies in Golf Cart control algorithms, Linux environments, MySQL, and Unity." } 
                          else { "開始進實驗室。學習高爾夫球車控制、Linux 系統、MySQL 資料庫以及 Unity 模擬環境。" } }
                    </p>
                </div>
            </div>
        </div>
    }
}
