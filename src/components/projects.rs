use yew::prelude::*;
use crate::Language;

#[function_component(Projects)]
pub fn projects(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    let card_style = "padding: 30px; margin-bottom: 20px; transition: 0.4s;";
    html! {
        <div class="container" style="padding: 100px 24px;">
            <h2 style="font-size: 2.5rem; font-weight: 900; margin-bottom: 50px; color: var(--primary);">{ if is_en { "TECHNICAL_EXPERTISE" } else { "技術專案" } }</h2>
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 30px;">
                <div class="glass" style={card_style}>
                    <h3 style="margin-bottom: 15px; font-weight: 900;">{ "NVIDIA GTC 2026" }</h3>
                    <p style="color: #aaa; font-size: 0.95rem;">{ if is_en { "Campus inspection system using Digital Twins. Personally signed by Jensen Huang." } else { "基於數位孿生的校園巡檢系統。榮獲黃仁勳親筆簽名肯定。" } }</p>
                </div>
                <div class="glass" style={card_style}>
                    <h3 style="margin-bottom: 15px; font-weight: 900;">{ "EMBODIED AI" }</h3>
                    <p style="color: #aaa; font-size: 0.95rem;">{ if is_en { "Deployment of NemoClaw and MGX Server optimization." } else { "NemoClaw 部署與 MGX Server 架構優化。" } }</p>
                </div>
            </div>
        </div>
    }
}
