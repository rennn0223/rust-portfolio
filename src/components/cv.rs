use yew::prelude::*;
use crate::Language;

#[function_component(Cv)]
pub fn cv(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    let timeline_item = "margin-bottom: 60px; position: relative; border-left: 2px solid rgba(59, 130, 246, 0.4); padding-left: 40px;";
    let dot = "position: absolute; left: -9px; top: 8px; width: 16px; height: 16px; background: var(--primary); border-radius: 50%; box-shadow: 0 0 15px var(--primary); border: 3px solid var(--bg);";
    let date_style = "font-family: 'JetBrains Mono', monospace; color: var(--primary); font-weight: 700; font-size: 0.95rem; margin-bottom: 8px;";

    html! {
        <div class="container" style="padding: 120px 24px;">
            <h2 style="font-size: 3.5rem; font-weight: 900; margin-bottom: 60px; letter-spacing: -2px;">
                { if is_en { "LOG_CHRONICLE" } else { "精確任務時序" } }
            </h2>
            
            <div style="max-width: 950px;">
                /* Phase 6: 現在 - 部署與優化 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style={date_style}>{ "2026.03.30 - PRESENT" }</div>
                    <h3 style="font-size: 1.6rem; margin-bottom: 12px;">{ if is_en { "MGX Infrastructure & NemoClaw Optimization" } else { "MGX 基礎設施與 NemoClaw AI 部署" } }</h3>
                    <ul style="color: #bbb; line-height: 1.8; font-size: 1rem;">
                        <li>{ if is_en { "Managing MGX Server hardware upgrades and specialized environment deployment." } else { "負責 MGX Server 硬體升級、系統環境建置及各項應用部署。" } }</li>
                        <li>{ if is_en { "Integrating NemoClaw on DGX Spark to bridge the user-experience gap in Omniverse via LLMs." } else { "於 DGX Spark 部署 NemoClaw，利用大型語言模型解決新手在 Omniverse 上的操作門檻。" } }</li>
                    </ul>
                </div>

                /* Phase 5: GTC 巔峰與交流 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style={date_style}>{ "2026.03.05 - 2026.03.21" }</div>
                    <h3 style="font-size: 1.6rem; margin-bottom: 12px;">{ if is_en { "NVIDIA GTC 2026 Exhibition & Berkeley Visit" } else { "NVIDIA GTC 2026 參展與 UC Berkeley 訪談" } }</h3>
                    <p style="color: #bbb; line-height: 1.7; margin-bottom: 10px;">
                        { if is_en { 
                            "Represented MSI at DGX Spark booth, exhibiting Omniverse Campus Inspection and Ackermann DT. Received personal recognition from Jensen Huang. Conducted technical exchanges at UC Berkeley BAIR (AI Autonomous Center) post-exhibition." 
                        } else { 
                            "代表微星於 DGX Spark 攤位展出校園巡檢及阿克曼小車數位孿生專案。獲得執行長黃仁勳親筆簽名肯定。展後受邀至 UC Berkeley BAIR (AI 自駕中心) 參觀並進行深度技術討論。" 
                        } }
                    </p>
                    <div style="color: var(--accent); font-weight: bold; font-size: 0.85rem;">{ "✓ Certified: NVIDIA DLI - Isaac for Accelerated Robotics" }</div>
                </div>

                /* Phase 4: 實作與攻堅 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style={date_style}>{ "2026.01.01 - 2026.02.28" }</div>
                    <h3 style="font-size: 1.6rem; margin-bottom: 12px;">{ if is_en { "MSI HQ In-house Collaboration" } else { "微星總部協同開發與 Alpamayo 研究" } }</h3>
                    <p style="color: #bbb; line-height: 1.7;">
                        { if is_en { 
                            "Resident developer at MSI HQ (Zhonghe). Collaborated with Senior Manager Shi-Zhe Hung and RD Yan-Cheng Lai. Focused on NVIDIA Alpamayo SDK integration and real-time synchronization logic for GTC demo." 
                        } else { 
                            "進駐微星中和總部，與資深經理洪士哲及 RD 賴彥成協同實作。專注於 NVIDIA Alpamayo 應用研究，並完成 GTC 參展用之校園巡檢與數位孿生系統開發。" 
                        } }
                    </p>
                </div>

                /* Phase 3: 感測器與自動化 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style={date_style}>{ "2025.08.30 - 2025.12.31" }</div>
                    <h3 style="font-size: 1.6rem; margin-bottom: 12px;">{ if is_en { "ROS2 Control & UAV Training" } else { "ROS2 控制系統與感測器融合" } }</h3>
                    <ul style="color: #bbb; line-height: 1.8;">
                        <li>{ if is_en { "Deployed ROS2 control for Ackermann platforms. Integrated LiDAR, IMU, and Camera arrays." } else { "實作 ROS2 控制架構。完成 LiDAR、IMU、Camera 等感測器在阿克曼底盤上的整合與標定。" } }</li>
                        <li>{ if is_en { "2025.10.30: Completed DJI Drone Professional Training Program." } else { "2025.10.30: 參加大疆無人機培訓課程並順利結業獲得證明。" } }</li>
                    </ul>
                </div>

                /* Phase 2: 合作案啟動 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style={date_style}>{ "2025.06.30 - 2025.08.29" }</div>
                    <h3 style="font-size: 1.6rem; margin-bottom: 12px;">{ if is_en { "NVIDIA Omniverse x MSI Strategic Partnership" } else { "NVIDIA Omniverse 與微星合作案啟動" } }</h3>
                    <p style="color: #bbb; line-height: 1.7;">
                        { if is_en { 
                            "Initiated the Omniverse Industrial Digital Twin project with MSI Innovation Center. Defined hardware-software interface standards for future simulation-ready infrastructure." 
                        } else { 
                            "正式開啟 NVIDIA Omniverse 與 MSI 創新前瞻中心的合作案。定義未來模擬就緒 (Simulation-ready) 基礎設施的軟硬體接口標準。" 
                        } }
                    </p>
                </div>

                /* Phase 1: 基礎養成 */
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style={date_style}>{ "2025.04.30 - 2025.06.29" }</div>
                    <h3 style="font-size: 1.6rem; margin-bottom: 12px;">{ if is_en { "Foundational Engineering & Lab Research" } else { "實驗室基礎建設與控制系統學習" } }</h3>
                    <p style="color: #bbb; line-height: 1.7;">
                        { if is_en { 
                            "Explored Golf Cart autonomous control. Mastered Linux environment, MySQL database management, and Unity-based simulation basics." 
                        } else { 
                            "投入高爾夫球車控制研究。掌握 Linux 系統運維、MySQL 資料庫管理及基於 Unity 的模擬環境搭建。" 
                        } }
                    </p>
                </div>
            </div>
        </div>
    }
}
