use yew::prelude::*;
use crate::Language;

#[function_component(Certificates)]
pub fn certificates(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    
    // 用於控制圖片預覽的狀態 (存儲圖片的路徑，None 代表關閉)
    let preview_img = use_state(|| None::<String>);

    // 關閉預覽的動作
    let close_modal = {
        let preview_img = preview_img.clone();
        Callback::from(move |_| preview_img.set(None))
    };

    let card_class = "glass";
    let card_style = "padding: 30px; display: flex; flex-direction: column; gap: 12px; border-left: 4px solid var(--primary);";
    let tag_style = "font-family: 'JetBrains Mono', monospace; font-size: 0.7rem; color: var(--accent); border: 1px solid var(--accent); padding: 3px 8px; border-radius: 4px; width: fit-content;";
    let link_btn = "margin-top: auto; display: inline-block; background: rgba(59, 130, 246, 0.1); color: var(--primary); border: 1px solid var(--primary); padding: 8px 16px; border-radius: 4px; text-decoration: none; font-family: 'JetBrains Mono'; font-size: 0.8rem; font-weight: bold; transition: 0.3s; text-align: center; cursor: pointer;";

    html! {
        <div class="container" style="padding: 100px 24px;">
            <h2 style="font-size: clamp(2.5rem, 8vw, 3.5rem); font-weight: 900; margin-bottom: 50px; letter-spacing: -2px;">
                { if is_en { "SYSTEM_CREDENTIALS" } else { "專業認證與授權" } }
            </h2>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 30px;">
                
                /* NVIDIA DLI: Isaac */
                <div class={card_class} style={card_style}>
                    <span style={tag_style}>{ "NVIDIA_DLI" }</span>
                    <h3 style="font-size: 1.4rem; font-weight: 900;">{ "Isaac for Accelerated Robotics" }</h3>
                    <p style="color: #aaa; font-size: 0.9rem;">{ if is_en { "Official certification for deploying AI robotics workflows using NVIDIA Isaac." } else { "NVIDIA 官方認證，掌握使用 Isaac 部署 AI 機器人工作流之核心能力。" } }</p>
                    <a href="https://learn.nvidia.com/certificates?id=AOuaSDlrRjSNIw37SgD1VQ" target="_blank" style={link_btn}>{ if is_en { "> VERIFY_CREDENTIAL" } else { "> 官方系統驗證" } }</a>
                </div>

                /* NVIDIA DLI: OpenUSD */
                <div class={card_class} style={card_style}>
                    <span style={tag_style}>{ "NVIDIA_DLI" }</span>
                    <h3 style="font-size: 1.4rem; font-weight: 900;">{ "OpenUSD: Stages, Prims & Attributes" }</h3>
                    <p style="color: #aaa; font-size: 0.9rem;">{ if is_en { "Mastery of Universal Scene Description for Omniverse Digital Twin environments." } else { "精通通用場景描述 (USD)，用於建構 Omniverse 數位孿生底層環境。" } }</p>
                    <a href="https://learn.nvidia.com/certificates?id=1DHB-ztRROWGqdjyu6qqTQ" target="_blank" style={link_btn}>{ if is_en { "> VERIFY_CREDENTIAL" } else { "> 官方系統驗證" } }</a>
                </div>

                /* NVIDIA DLI: Jetson */
                <div class={card_class} style={card_style}>
                    <span style={tag_style}>{ "NVIDIA_DLI" }</span>
                    <h3 style="font-size: 1.4rem; font-weight: 900;">{ "AI on Jetson Nano" }</h3>
                    <p style="color: #aaa; font-size: 0.9rem;">{ if is_en { "Deployment of deep learning models on Edge AI hardware architectures." } else { "具備在 Edge AI 邊緣硬體架構上部署深度學習模型之能力。" } }</p>
                    <a href="https://learn.nvidia.com/certificates?id=EN5-FdNJT_KR9akW3bacrg" target="_blank" style={link_btn}>{ if is_en { "> VERIFY_CREDENTIAL" } else { "> 官方系統驗證" } }</a>
                </div>

                /* Duolingo */
                <div class={card_class} style={card_style}>
                    <span style="font-family: 'JetBrains Mono', monospace; font-size: 0.7rem; color: #f59e0b; border: 1px solid #f59e0b; padding: 3px 8px; border-radius: 4px; width: fit-content;">{ "LANGUAGE_PRO" }</span>
                    <h3 style="font-size: 1.4rem; font-weight: 900;">{ "Duolingo English Test" }</h3>
                    <p style="color: #aaa; font-size: 0.9rem;">{ if is_en { "Certified English proficiency for international technical communication." } else { "國際英語能力認證，具備流暢的跨國技術溝通能力。" } }</p>
                    <a href="https://certs.duolingo.com/tlegwwbno75h9itb" target="_blank" style={link_btn}>{ if is_en { "> VIEW_CERTIFICATE" } else { "> 查看成績證明" } }</a>
                </div>

                /* DJI Drone (範例：這個我們用 Modal 圖片預覽) */
                <div class={card_class} style={card_style}>
                    <span style={tag_style}>{ "HARDWARE_OP" }</span>
                    <h3 style="font-size: 1.4rem; font-weight: 900;">{ "DJI Drone Professional Training" }</h3>
                    <p style="color: #aaa; font-size: 0.9rem;">{ if is_en { "Certified operational proficiency for commercial UAV deployment." } else { "大疆專業無人機操作培訓，具備商用無人機部署與操作能力。" } }</p>
                    /* 這裡示範點擊按鈕後，觸發預覽圖片的 Modal */
                    /* (如果你有圖片，把 "assets/dji_cert.jpeg" 換成你的檔名；如果沒有，可以先留著測試功能) */
                    <button 
                        onclick={{ let preview_img = preview_img.clone(); Callback::from(move |_| preview_img.set(Some("assets/GTC2026.jpeg".to_string()))) }} 
                        style={link_btn}>
                        { if is_en { "[ ] PREVIEW_DOCUMENT" } else { "[ ] 預覽授權文件" } }
                    </button>
                </div>
            </div>

            /* 彈出視窗 (Modal) 邏輯：如果有圖片路徑，就渲染這塊 HTML */
            if let Some(img_src) = (*preview_img).clone() {
                <div class="modal-overlay" onclick={close_modal.clone()}>
                    <div class="modal-content" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                        <button class="modal-close" onclick={close_modal}>{ "×" }</button>
                        <img src={img_src} alt="Certificate Preview" style="max-width: 100%; max-height: 85vh; border-radius: 6px; display: block;" />
                    </div>
                </div>
            }
        </div>
    }
}
