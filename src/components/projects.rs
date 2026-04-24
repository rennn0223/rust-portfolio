use yew::prelude::*;
use crate::Language;

#[function_component(Projects)]
pub fn projects(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    html! {
        <div class="container" style="padding: 100px 24px;">
            <h2 style="font-size: 3rem; font-weight: 900; margin-bottom: 60px; letter-spacing: -2px;">{ if is_en { "TECHNICAL_EXPERTISE" } else { "深度專案內容" } }</h2>
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 30px;">
                <div class="glass" style="padding: 40px;">
                    <div style="font-size: 0.8rem; color: var(--primary); margin-bottom: 10px;">{ "NVIDIA_EVENT_2026" }</div>
                    <h3 style="margin-bottom: 20px; font-size: 1.5rem;">{ "NVIDIA GTC EXHIBITOR" }</h3>
                    <p style="color: #aaa; line-height: 1.7;">
                        { if is_en { "Lead developer of the Alpamayo-based Digital Twin system. Successfully showcased real-time campus inspection tools to NVIDIA's executive team." } 
                          else { "領銜開發基於 Alpamayo 的數位孿生系統，成功向 NVIDIA 高層展示即時校園巡檢技術。" } }
                    </p>
                </div>
                <div class="glass" style="padding: 40px;">
                    <div style="font-size: 0.8rem; color: var(--primary); margin-bottom: 10px;">{ "EMBODIED_AI_LAB" }</div>
                    <h3 style="margin-bottom: 20px; font-size: 1.5rem;">{ "SYSTEMS OPTIMIZATION" }</h3>
                    <p style="color: #aaa; line-height: 1.7;">
                        { if is_en { "Architecting high-performance backends for NemoClaw and MGX Servers. Focused on low-latency Wasm-based inference pipelines." } 
                          else { "構建 NemoClaw 與 MGX Server 的高效能後端。專注於基於 Wasm 的低延遲推理流水線。" } }
                    </p>
                </div>
                <div class="glass" style="padding: 40px;">
                    <div style="font-size: 0.8rem; color: var(--primary); margin-bottom: 10px;">{ "RUST_ENGINEERING" }</div>
                    <h3 style="margin-bottom: 20px; font-size: 1.5rem;">{ "WASM INFRASTRUCTURE" }</h3>
                    <p style="color: #aaa; line-height: 1.7;">
                        { if is_en { "Developing secure, sandboxed execution environments for edge computing devices using Rust toolchains." } 
                          else { "利用 Rust 工具鏈為邊緣運算設備開發安全且隔離的沙盒執行環境。" } }
                    </p>
                </div>
            </div>
        </div>
    }
}
