use yew::prelude::*;
use crate::Language;

#[function_component(Cv)]
pub fn cv(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    let timeline_item = "margin-bottom: 60px; position: relative; border-left: 2px solid rgba(59,130,246,0.3); padding-left: 35px;";
    let dot = "position: absolute; left: -9px; top: 8px; width: 16px; height: 16px; background: var(--primary); border-radius: 50%; box-shadow: 0 0 12px var(--primary); border: 3px solid var(--bg);";
    let btn_style = "display: inline-block; margin: 10px 10px 0 0; color: var(--accent); border: 1px solid var(--accent); padding: 6px 14px; border-radius: 4px; text-decoration: none; font-size: 0.75rem; font-family: 'JetBrains Mono'; transition: 0.3s; background: rgba(16, 185, 129, 0.05);";

    html! {
        <div class="container" style="padding: 120px 24px;">
            <div style="margin-bottom: 50px; display: flex; flex-wrap: wrap; gap: 15px;">
                <a href="https://www.linkedin.com/in/rennn223" target="_blank" style={btn_style}>{ "LINKEDIN ↗" }</a>
                <a href="https://certs.duolingo.com/tlegwwbno75h9itb" target="_blank" style={btn_style}>{ "DUOLINGO_PRO ↗" }</a>
            </div>

            <h2 style="font-size: 3rem; font-weight: 900; margin-bottom: 60px;">{ if is_en { "SYSTEM_LOG" } else { "系統演進日誌" } }</h2>
            
            <div style="max-width: 900px;">
                <div style={timeline_item}>
                    <div style={dot}></div>
                    <div style="color: var(--primary); font-family: 'JetBrains Mono'; font-weight: bold;">{ "2026.03 - PRESENT" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "MGX & NemoClaw Deployment" } else { "MGX 與 NemoClaw 部署" } }</h3>
                    <div style="margin-top: 15px; display: flex; flex-wrap: wrap; gap: 10px;">
                        <a href="https://learn.nvidia.com/certificates?id=AOuaSDlrRjSNIw37SgD1VQ" target="_blank" style={btn_style}>{ "ISAAC ROBOTICS ↗" }</a>
                        <a href="https://learn.nvidia.com/certificates?id=1DHB-ztRROWGqdjyu6qqTQ" target="_blank" style={btn_style}>{ "OPENUSD ↗" }</a>
                        <a href="https://learn.nvidia.com/certificates?id=EN5-FdNJT_KR9akW3bacrg" target="_blank" style={btn_style}>{ "JETSON AI ↗" }</a>
                    </div>
                </div>

                <div style={timeline_item}>
                    <div style={dot}></div>
                    <img src="assets/GTC2026.jpg" alt="GTC 2026" style="width: 100%; max-width: 600px; border-radius: 12px; margin: 20px 0; border: 1px solid rgba(255,255,255,0.1);" />
                    <div style="color: var(--primary); font-family: 'JetBrains Mono';">{ "2026.03" }</div>
                    <h3 style="margin: 10px 0;">{ if is_en { "NVIDIA GTC Exhibition" } else { "NVIDIA GTC 參展實錄" } }</h3>
                    <p style="color: #bbb; line-height: 1.8;">{ if is_en { "Technical discussions at UC Berkeley BAIR following the GTC exhibition." } else { "於微星攤位展出數位孿生專案，並受邀至 UC Berkeley BAIR 進行技術交流。" } }</p>
                </div>
            </div>
        </div>
    }
}
