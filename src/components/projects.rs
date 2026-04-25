use yew::prelude::*;
use crate::Language;

#[function_component(Projects)]
pub fn projects(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    
    // 為三個專案建立獨立的展開狀態
    let exp_1 = use_state(|| false);
    let exp_2 = use_state(|| false);
    let exp_3 = use_state(|| false);

    let toggle_1 = { let exp_1 = exp_1.clone(); Callback::from(move |_| exp_1.set(!*exp_1)) };
    let toggle_2 = { let exp_2 = exp_2.clone(); Callback::from(move |_| exp_2.set(!*exp_2)) };
    let toggle_3 = { let exp_3 = exp_3.clone(); Callback::from(move |_| exp_3.set(!*exp_3)) };

    let card_class = "glass";
    let card_style = "padding: 25px; display: flex; flex-direction: column; gap: 15px; border-radius: 16px; transition: all 0.3s ease;";
    let img_style = "width: 100%; height: 220px; object-fit: cover; border-radius: 12px; border: 1px solid rgba(255,255,255,0.1); background: #111;";
    let tag_style = "font-family: 'JetBrains Mono', monospace; font-size: 0.65rem; color: var(--primary); border: 1px solid rgba(59, 130, 246, 0.3); padding: 3px 8px; border-radius: 4px;";
    let btn_style = "margin-top: auto; align-self: flex-start; cursor: pointer; background: transparent; border: 1px solid var(--primary); color: var(--primary); padding: 6px 16px; border-radius: 20px; font-family: 'JetBrains Mono'; font-size: 0.75rem; transition: 0.3s;";

    html! {
        <div class="container" style="padding: 100px 24px;">
            <h2 style="font-size: clamp(2.5rem, 8vw, 3.5rem); font-weight: 900; margin-bottom: 50px; letter-spacing: -2px;">
                { if is_en { "MISSION_GALLERY" } else { "技術專案實錄" } }
            </h2>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 30px; align-items: start;">
                
                /* 專案 1: GTC 2026 */
                <div class={card_class} style={card_style}>
                    <img src="assets/signiture.jpeg" alt="Jensen Huang Signature" style={img_style} />
                    <div style="display: flex; flex-wrap: wrap; gap: 8px;">
                        <span style={tag_style}>{ "NVIDIA_GTC" }</span><span style={tag_style}>{ "SIGNATURE" }</span>
                    </div>
                    <h3 style="font-size: 1.5rem; font-weight: 900; color: #fff;">{ if is_en { "GTC 2026: Ackermann DT" } else { "GTC 2026：阿克曼數位孿生" } }</h3>
                    <p style="color: #aaa; font-size: 0.95rem; line-height: 1.6;">
                        { if is_en { "Exhibited at DGX Spark. Features real-time Alpamayo sync logic." } else { "於 DGX Spark 攤位展出。整合 Alpamayo 實時同步邏輯。" } }
                    </p>
                    if *exp_1 {
                        <div style="margin-top: 10px; padding-top: 15px; border-top: 1px solid rgba(255,255,255,0.1); color: #ccc; font-size: 0.9rem; line-height: 1.7; animation: pageFade 0.4s forwards;">
                            { if is_en { 
                                "Lead Architect for the Campus Inspection System. Developed a real-time Digital Twin environment for autonomous monitoring using Omniverse. Collaborated with MSI to deploy on DGX Spark, ensuring sub-millisecond synchronization. Personally signed and recognized by CEO Jensen Huang." 
                            } else { 
                                "擔任校園巡檢系統首席架構師。使用 Omniverse 開發用於自主監控的即時數位孿生環境。與微星團隊合作部署於 DGX Spark，確保亞毫秒級的虛實同步，專案榮獲執行長黃仁勳親筆簽名肯定。" 
                            } }
                        </div>
                    }
                    <button onclick={toggle_1} style={btn_style}>{ if *exp_1 { if is_en {"COLLAPSE_"} else {"收合內容_"} } else { if is_en {"VIEW_MORE_"} else {"查看更多_"} } }</button>
                </div>

                /* 專案 2: UC Berkeley BAIR */
                <div class={card_class} style={card_style}>
                    <img src="assets/UCBerkeley.jpeg" alt="UC Berkeley BAIR" style={img_style} />
                    <div style="display: flex; flex-wrap: wrap; gap: 8px;">
                        <span style={tag_style}>{ "RESEARCH" }</span><span style={tag_style}>{ "AUTONOMOUS" }</span>
                    </div>
                    <h3 style="font-size: 1.5rem; font-weight: 900; color: #fff;">{ if is_en { "UC Berkeley BAIR Visit" } else { "柏克萊 AI 自駕中心" } }</h3>
                    <p style="color: #aaa; font-size: 0.95rem; line-height: 1.6;">
                        { if is_en { "Technical discussion at the BAIR Lab regarding autonomous architectures." } else { "受邀至 UC Berkeley BAIR 實驗室，針對自動駕駛架構進行學術交流。" } }
                    </p>
                    if *exp_2 {
                        <div style="margin-top: 10px; padding-top: 15px; border-top: 1px solid rgba(255,255,255,0.1); color: #ccc; font-size: 0.9rem; line-height: 1.7; animation: pageFade 0.4s forwards;">
                            { if is_en { 
                                "Invited to the Berkeley Artificial Intelligence Research (BAIR) Lab post-GTC 2026. Conducted in-depth architectural reviews on autonomous driving systems, focusing on ROS2 sensor fusion (LiDAR, IMU) and Wasm deployment for Embodied AI edge computing." 
                            } else { 
                                "GTC 展後受邀前往加州柏克萊大學人工智慧研究實驗室 (BAIR)。針對自動駕駛系統進行深度架構審查，重點討論 ROS2 多感測器融合 (LiDAR, IMU) 以及具身智能 (Embodied AI) 在邊緣運算上的 Wasm 部署策略。" 
                            } }
                        </div>
                    }
                    <button onclick={toggle_2} style={btn_style}>{ if *exp_2 { if is_en {"COLLAPSE_"} else {"收合內容_"} } else { if is_en {"VIEW_MORE_"} else {"查看更多_"} } }</button>
                </div>

                /* 專案 3: MSI Collaboration */
                <div class={card_class} style={card_style}>
                    <img src="assets/MSI.jpeg" alt="MSI Collaboration" style={img_style} />
                    <div style="display: flex; flex-wrap: wrap; gap: 8px;">
                        <span style={tag_style}>{ "INDUSTRIAL" }</span><span style={tag_style}>{ "OMNIVERSE" }</span>
                    </div>
                    <h3 style="font-size: 1.5rem; font-weight: 900; color: #fff;">{ if is_en { "MSI HQ Integration" } else { "微星總部協同開發" } }</h3>
                    <p style="color: #aaa; font-size: 0.95rem; line-height: 1.6;">
                        { if is_en { "Resident implementation at MSI Innovation Center. Developed Omniverse environments." } else { "進駐微星創新前瞻中心，實作 Omniverse 校園巡檢系統。" } }
                    </p>
                    if *exp_3 {
                        <div style="margin-top: 10px; padding-top: 15px; border-top: 1px solid rgba(255,255,255,0.1); color: #ccc; font-size: 0.9rem; line-height: 1.7; animation: pageFade 0.4s forwards;">
                            { if is_en { 
                                "Embedded at MSI Headquarters to architect industrial-grade Digital Twin pipelines. Managed NVIDIA MGX Server hardware upgrades, deployed NemoClaw on DGX Spark for LLM-driven automation, and established Edge-AI protocols to lower the entry barrier for developers." 
                            } else { 
                                "進駐微星中和總部，建構工業級數位孿生流水線。負責 NVIDIA MGX 伺服器硬體升級，並於 DGX Spark 部署 NemoClaw 藉由 LLM 驅動自動化。建立基於邊緣 AI 的通訊協定，大幅降低新手開發 Omniverse 的技術門檻。" 
                            } }
                        </div>
                    }
                    <button onclick={toggle_3} style={btn_style}>{ if *exp_3 { if is_en {"COLLAPSE_"} else {"收合內容_"} } else { if is_en {"VIEW_MORE_"} else {"查看更多_"} } }</button>
                </div>
            </div>
        </div>
    }
}
