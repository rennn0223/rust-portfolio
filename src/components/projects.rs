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
                { if is_en { "SELECTED_RECORDS" } else { "精選技術專案" } }
            </h2>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(350px, 1fr)); gap: 40px;">
                /* 專案 1: GTC 2026 */
                <div class={card_class} style={card_style}>
                    <div style="display: flex; gap: 10px;">
                        <span style={tag_style}>{ "NVIDIA_OMNIVERSE" }</span>
                        <span style={tag_style}>{ "DIGITAL_TWINS" }</span>
                    </div>
                    <h3 style="font-size: 1.8rem; font-weight: 900;">{ if is_en { "NVIDIA GTC 2026 Exhibitor" } else { "NVIDIA GTC 2026 參展專案" } }</h3>
                    <p style="color: #aaa; line-height: 1.8; font-size: 1rem;">
                        { if is_en { 
                            "Lead Architect for the Alpamayo-based Campus Inspection System. Developed a real-time Digital Twin environment for autonomous monitoring. Personally recognized and signed by NVIDIA CEO Jensen Huang during the event." 
                        } else { 
                            "擔任基於 Alpamayo 的校園巡檢系統首席架構師。開發實時數位孿生環境用於自主監控，該專案於 GTC 2026 展出，並榮獲 NVIDIA 執行長黃仁勳親筆簽名肯定。" 
                        } }
                    </p>
                    <div style="color: var(--accent); font-size: 0.9rem; font-weight: bold;">{ "Tech: Rust, Wasm, USD, Alpamayo API" }</div>
                </div>

                /* 專案 2: Embodied AI */
                <div class={card_class} style={card_style}>
                    <div style="display: flex; gap: 10px;">
                        <span style={tag_style}>{ "EMBODIED_AI" }</span>
                        <span style={tag_style}>{ "MGX_SERVER" }</span>
                    </div>
                    <h3 style="font-size: 1.8rem; font-weight: 900;">{ if is_en { "NemoClaw Deployment & Optimization" } else { "NemoClaw 部署與系統優化" } }</h3>
                    <p style="color: #aaa; line-height: 1.8; font-size: 1rem;">
                        { if is_en { 
                            "Architected high-concurrency inference pipelines for NemoClaw within Omniverse. Optimized MGX Server configurations to reduce end-to-end latency in embodied AI applications by 35%." 
                        } else { 
                            "為 Omniverse 中的 NemoClaw 構建高併發推理流水線。優化 MGX Server 配置，使具身智能應用端的端到端延遲降低了 35%。" 
                        } }
                    </p>
                    <div style="color: var(--accent); font-size: 0.9rem; font-weight: bold;">{ "Tech: CUDA, Python, NVIDIA MGX, Nemo" }</div>
                </div>

                /* 專案 3: MSI Collaboration */
                <div class={card_class} style={card_style}>
                    <div style="display: flex; gap: 10px;">
                        <span style={tag_style}>{ "INDUSTRIAL_AI" }</span>
                        <span style={tag_style}>{ "EDGE_COMPUTING" }</span>
                    </div>
                    <h3 style="font-size: 1.8rem; font-weight: 900;">{ if is_en { "MSI Innovation - Industrial DT" } else { "微星創新中心 - 工業數位孿生" } }</h3>
                    <p style="color: #aaa; line-height: 1.8; font-size: 1rem;">
                        { if is_en { 
                            "Collaborated with MSI Innovation Center to develop next-gen industrial inspection infrastructure. Focused on scaling Wasm-based edge inference for factory automation." 
                        } else { 
                            "與微星科技創新前瞻中心合作，開發下一代工業巡檢基礎設施。專注於為工廠自動化擴展基於 Wasm 的邊緣端推理技術。" 
                        } }
                    </p>
                    <div style="color: var(--accent); font-size: 0.9rem; font-weight: bold;">{ "Tech: Rust, Wasm, EdgeRuntime" }</div>
                </div>
            </div>
        </div>
    }
}
