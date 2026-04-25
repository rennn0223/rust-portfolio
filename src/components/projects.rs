use yew::prelude::*;
use crate::Language;

#[function_component(Projects)]
pub fn projects(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    let card_class = "glass";
    let card_style = "padding: 25px; display: flex; flex-direction: column; gap: 15px; height: 100%;";
    let img_style = "width: 100%; height: 240px; object-fit: cover; border-radius: 8px; border: 1px solid rgba(255,255,255,0.1); background: #111;";
    let tag_style = "font-family: 'JetBrains Mono', monospace; font-size: 0.65rem; color: var(--primary); border: 1px solid rgba(59, 130, 246, 0.3); padding: 3px 8px; border-radius: 4px;";

    html! {
        <div class="container" style="padding: 100px 24px;">
            <h2 style="font-size: clamp(2.5rem, 8vw, 3.5rem); font-weight: 900; margin-bottom: 50px; letter-spacing: -2px;">
                { if is_en { "MISSION_GALLERY" } else { "技術專案實錄" } }
            </h2>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 30px;">
                /* GTC 2026 */
                <div class={card_class} style={card_style}>
                    <img src="assets/signiture.jpg" alt="Jensen Huang Signature" style={img_style} />
                    <div style="display: flex; flex-wrap: wrap; gap: 8px;">
                        <span style={tag_style}>{ "NVIDIA_GTC" }</span>
                        <span style={tag_style}>{ "SIGNATURE" }</span>
                    </div>
                    <h3 style="font-size: 1.5rem; font-weight: 900; color: #fff;">{ if is_en { "GTC 2026: Ackermann DT" } else { "GTC 2026：阿克曼數位孿生" } }</h3>
                    <p style="color: #aaa; font-size: 0.95rem; line-height: 1.6;">
                        { if is_en { "Exhibited at DGX Spark. Features real-time Alpamayo sync logic. Personally signed by CEO Jensen Huang." } 
                          else { "於 DGX Spark 攤位展出。整合 Alpamayo 實時同步邏輯，專案榮獲執行長黃仁勳親筆簽名肯定。" } }
                    </p>
                </div>

                /* UC Berkeley BAIR */
                <div class={card_class} style={card_style}>
                    <img src="assets/UCBerkeley.jpg" alt="UC Berkeley BAIR" style={img_style} />
                    <div style="display: flex; flex-wrap: wrap; gap: 8px;">
                        <span style={tag_style}>{ "RESEARCH" }</span>
                        <span style={tag_style}>{ "AUTONOMOUS" }</span>
                    </div>
                    <h3 style="font-size: 1.5rem; font-weight: 900; color: #fff;">{ if is_en { "UC Berkeley BAIR Visit" } else { "柏克萊 AI 自駕中心" } }</h3>
                    <p style="color: #aaa; font-size: 0.95rem; line-height: 1.6;">
                        { if is_en { "Technical discussion at the BAIR Lab regarding autonomous driving architectures and robotics synchronization." } 
                          else { "受邀至 UC Berkeley BAIR 實驗室，針對自動駕駛架構與機器人同步技術進行深度學術交流。" } }
                    </p>
                </div>

                /* MSI Collaboration */
                <div class={card_class} style={card_style}>
                    <img src="assets/MSI.jpg" alt="MSI Collaboration" style={img_style} />
                    <div style="display: flex; flex-wrap: wrap; gap: 8px;">
                        <span style={tag_style}>{ "INDUSTRIAL" }</span>
                        <span style={tag_style}>{ "OMNIVERSE" }</span>
                    </div>
                    <h3 style="font-size: 1.5rem; font-weight: 900; color: #fff;">{ if is_en { "MSI HQ Integration" } else { "微星總部協同開發" } }</h3>
                    <p style="color: #aaa; font-size: 0.95rem; line-height: 1.6;">
                        { if is_en { "Resident implementation at MSI Innovation Center. Developed Omniverse Campus Patrol and MGX configurations." } 
                          else { "進駐微星創新前瞻中心，實作 Omniverse 校園巡檢系統並優化 MGX 伺服器運算環境。" } }
                    </p>
                </div>
            </div>
        </div>
    }
}
