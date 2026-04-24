use yew::prelude::*;
use crate::Language;

#[function_component(Projects)]
pub fn projects(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    let card_class = "glass";
    let card_style = "padding: 30px; display: flex; flex-direction: column; gap: 15px; overflow: hidden;";
    let img_container = "margin: 15px -30px -30px -30px; border-top: 1px solid rgba(255,255,255,0.1); position: relative;";
    let tag_style = "font-family: 'JetBrains Mono', monospace; font-size: 0.7rem; color: var(--primary); border: 1px solid var(--primary); padding: 2px 8px; border-radius: 4px;";

    html! {
        <div class="container" style="padding: 120px 24px;">
            <h2 style="font-size: 3.5rem; font-weight: 900; margin-bottom: 60px; letter-spacing: -2px;">
                { if is_en { "MISSION_GALLERY" } else { "技術專案展示" } }
            </h2>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(350px, 1fr)); gap: 40px;">
                /* GTC 2026 專案 */
                <div class={card_class} style={card_style}>
                    <div style="display: flex; gap: 8px;">
                        <span style={tag_style}>{ "NVIDIA_GTC" }</span>
                        <span style={tag_style}>{ "SIGNATURE" }</span>
                    </div>
                    <h3 style="font-size: 1.5rem; font-weight: 900;">{ if is_en { "Omniverse Campus & Ackermann DT" } else { "校園巡檢與阿克曼小車數位孿生" } }</h3>
                    <p style="color: #aaa; font-size: 0.9rem; line-height: 1.6;">
                        { if is_en { "Showcased at DGX Spark. Features NVIDIA Alpamayo sync logic. Personally signed by CEO Jensen Huang." } 
                          else { "於 DGX Spark 攤位展出。整合 Alpamayo 同步邏輯，獲得執行長黃仁勳親筆簽名肯定。" } }
                    </p>
                    /* 圖片預留位：請將圖片放進 assets 並修改下方的 src */
                    <div style={img_container}>
                        <img src="assets/gtc_exhibition.jpg" alt="GTC Exhibition" style="width: 100%; height: 200px; object-fit: cover; opacity: 0.8;" />
                        <div style="position: absolute; bottom: 10px; right: 10px; background: var(--primary); color: #fff; font-size: 0.6rem; padding: 2px 6px;">{ "CERTIFIED BY JENSEN HUANG" }</div>
                    </div>
                </div>

                /* MGX Server 專案 */
                <div class={card_class} style={card_style}>
                    <div style="display: flex; gap: 8px;">
                        <span style={tag_style}>{ "MGX_SERVER" }</span>
                        <span style={tag_style}>{ "NEMOCLAW" }</span>
                    </div>
                    <h3 style="font-size: 1.5rem; font-weight: 900;">{ if is_en { "MGX Infrastructure & AI Deployment" } else { "MGX 伺服器與 AI 環境部署" } }</h3>
                    <p style="color: #aaa; font-size: 0.9rem; line-height: 1.6;">
                        { if is_en { "Managing server upgrades and NemoClaw integration to enhance Omniverse accessibility." } 
                          else { "負責伺服器升級與 NemoClaw 整合，降低 Omniverse 進入門檻。" } }
                    </p>
                    <div style={img_container}>
                        <img src="assets/mgx_server.jpg" alt="MGX Server" style="width: 100%; height: 200px; object-fit: cover; opacity: 0.8;" />
                    </div>
                </div>
            </div>
        </div>
    }
}
