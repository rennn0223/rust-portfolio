use yew::prelude::*;
use crate::Language;

#[function_component(Cv)]
pub fn cv(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    let timeline_item = "margin-bottom: 60px; position: relative; border-left: 2px solid rgba(59,130,246,0.3); padding-left: 35px;";
    let dot = "position: absolute; left: -9px; top: 8px; width: 16px; height: 16px; background: var(--primary); border-radius: 50%; box-shadow: 0 0 12px var(--primary); border: 3px solid #030303;";
    let btn_style = "display: inline-block; margin: 10px 10px 0 0; color: var(--accent); border: 1px solid rgba(16, 185, 129, 0.4); padding: 6px 14px; border-radius: 4px; text-decoration: none; font-size: 0.75rem; font-family: 'JetBrains Mono'; transition: 0.3s; background: rgba(16, 185, 129, 0.05);";

    html! {
        <div class="container" style="padding: 100px 24px;">
            <div style="margin-bottom: 50px; display: flex; flex-wrap: wrap; gap: 15px;">
                <a href="https://www.linkedin.com/in/rennn223" target="_blank" style={btn_style}>{ "LINKEDIN ↗" }</a>
                <a href="https://certs.duolingo.com/tlegwwbno75h9itb" target="_blank" style={btn_style}>{ "DUOLINGO_ENGLISH_PRO ↗" }</a>
            </div>

            <h2 style="font-size: 3rem; font-weight: 900; margin-bottom: 60px;">{ if is_en { "SYSTEM_LOG" } else { "系統演進日誌" } }</h2>
            
            <div style="max-width: 900px;">
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style="color: var(--primary); font-family: 'JetBrains Mono'; font-weight: bold;">{ "2026.03 - PRESENT" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "MGX Deployment & Professional Credentials" } else { "MGX 環境部署與專業認證" } }</h3>
                    <p style="color: #bbb; line-height: 1.8;">{ if is_en { "Managing MGX Server upgrades and installing NemoClaw on DGX Spark to lower user barriers." } else { "負責處理 MGX Server 硬體升級，並於 DGX Spark 部署 NemoClaw 解決方案。" } }</p>
                    <div style="margin-top: 15px; display: flex; flex-wrap: wrap;">
                        <a href="https://learn.nvidia.com/certificates?id=AOuaSDlrRjSNIw37SgD1VQ" target="_blank" style={btn_style}>{ "ISAAC ROBOTICS CERT ↗" }</a>
                        <a href="https://learn.nvidia.com/certificates?id=1DHB-ztRROWGqdjyu6qqTQ" target="_blank" style={btn_style}>{ "OPENUSD STAGES CERT ↗" }</a>
                        <a href="https://learn.nvidia.com/certificates?id=EN5-FdNJT_KR9akW3bacrg" target="_blank" style={btn_style}>{ "JETSON AI CERT ↗" }</a>
                    </div>
                </div>

                <div style={timeline_item}>
                    <div style={dot}></div>
                    // 這裡改成了 .jpeg
                    <img src="assets/GTC2026.jpeg" alt="GTC 2026" style="width: 100%; max-width: 600px; border-radius: 8px; margin: 15px 0; border: 1px solid rgba(255,255,255,0.1);" />
                    <div style="color: var(--primary); font-family: 'JetBrains Mono';">{ "2026.03" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "NVIDIA GTC Exhibition & BAIR Lab" } else { "NVIDIA GTC 參展與柏克萊技術交流" } }</h3>
                    <p style="color: #bbb; line-height: 1.8;">
                        { if is_en { "Showcased project at MSI DGX Spark booth. Engaged in technical discussions at UC Berkeley BAIR." } 
                          else { "於微星 DGX Spark 攤位展出校園巡檢專案。展後赴 UC Berkeley BAIR 進行自駕技術探討。" } }
                    </p>
                </div>

                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style="color: var(--primary); font-family: 'JetBrains Mono';">{ "2026.01 - 2026.02" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "MSI HQ Resident Implementation" } else { "微星總部協同實作期" } }</h3>
                    <p style="color: #bbb; line-height: 1.8;">
                        { if is_en { "Collaborated with Senior Manager Shi-Zhe Hung & RD Yan-Cheng Lai on Alpamayo integration for GTC." } 
                          else { "進駐微星中和總部，與洪士哲經理及 RD 賴彥成協同實作 Alpamayo 邏輯以備戰 GTC。" } }
                    </p>
                </div>

                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style="color: var(--primary); font-family: 'JetBrains Mono';">{ "2025.04 - 2025.12" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "Sensor Fusion & Foundations" } else { "感測器融合與開發基礎" } }</h3>
                    <ul style="color: #bbb; line-height: 1.8; padding-left: 20px;">
                        <li>{ if is_en { "ROS2 integration: LiDAR, IMU, and Camera arrays for Ackermann platform (Aug)." } else { "ROS2 控制開發：整合 LiDAR、IMU 與攝影機應用於阿克曼平台 (8月)。" } }</li>
                        <li>{ if is_en { "Completed DJI Drone Professional Training Program (Oct)." } else { "完成大疆無人機培訓課程並獲得專業操作證書 (10月)。" } }</li>
                        <li>{ if is_en { "Initiated MSI x Omniverse Strategic Partnership (Jun)." } else { "開啟微星與 Omniverse 戰略合作專案 (6月)。" } }</li>
                        <li>{ if is_en { "Lab entry: Golf Cart control, Linux, MySQL, Unity (Apr)." } else { "進入實驗室研究高爾夫球車控制，打底 Linux、MySQL 及 Unity (4月)。" } }</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
