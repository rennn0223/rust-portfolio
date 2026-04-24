use yew::prelude::*;
use crate::Language;

#[derive(Properties, PartialEq)]
pub struct PageProps { pub lang: Language }

#[function_component(Projects)]
pub fn projects(props: &PageProps) -> Html {
    let is_en = props.lang == Language::En;
    html! {
        <div class="container" style="padding-top: 120px; padding-bottom: 80px;">
            <h1 class="section-title">
                { if is_en { "Technical Deep Dives" } else { "技術特展與解析" } }
            </h1>

            // GTC 2026
            <div style="background: #111; border-radius: 12px; border: 1px solid rgba(255,255,255,0.05); margin-bottom: 40px; overflow: hidden;">
                <div style="background: #1a1a1a; padding: 35px; border-bottom: 1px solid rgba(255,255,255,0.05);">
                    <h3 style="font-size: 1.8rem; font-weight: 900;">{ if is_en { "Campus Inspection & Ackermann Digital Twin" } else { "校園巡檢與阿克曼小車數位孿生" } }</h3>
                </div>
                <div style="padding: 35px; color: #aaa; line-height: 1.8;">
                    { if is_en { "Exhibited at NVIDIA GTC 2026 California. Developed an autonomous campus inspection system using NVIDIA Alpamayo and Digital Twin technology. Personally signed by Jensen Huang." } 
                      else { "於美國加州 NVIDIA GTC 2026 現場展出。利用 NVIDIA Alpamayo 與數位孿生技術打造校園自主巡檢與阿克曼轉向小車系統，榮獲黃仁勳親筆簽名肯定。" } }
                </div>
            </div>

            // Infrastructure & NemoClaw
            <div style="background: #111; border-radius: 12px; border: 1px solid rgba(255,255,255,0.05); margin-bottom: 40px; overflow: hidden;">
                <div style="background: #1a1a1a; padding: 35px; border-bottom: 1px solid rgba(255,255,255,0.05);">
                    <h3 style="font-size: 1.8rem; font-weight: 900;">{ if is_en { "AI Infrastructure & NemoClaw" } else { "高階 AI 基礎設施與 NemoClaw 整合" } }</h3>
                </div>
                <div style="padding: 35px; color: #aaa; line-height: 1.8;">
                    { if is_en { "Managing MGX Server architecture and deploying NemoClaw on DGX Spark to lower technical barriers for Omniverse developers via LLM integration." } 
                      else { "負責 MGX Server 環境建置，並於 DGX Spark 部署 NemoClaw，透過整合大型語言模型 (LLM) 解決新手開發者進入 Omniverse 生態系的技術門檻。" } }
                </div>
            </div>
            
            // Sensor Fusion
            <div style="background: #111; border-radius: 12px; border: 1px solid rgba(255,255,255,0.05); margin-bottom: 40px; overflow: hidden;">
                <div style="background: #1a1a1a; padding: 35px; border-bottom: 1px solid rgba(255,255,255,0.05);">
                    <h3 style="font-size: 1.8rem; font-weight: 900;">{ if is_en { "Hardware-in-the-Loop & Sensor Fusion" } else { "實體自駕車控制與多感測器融合" } }</h3>
                </div>
                <div style="padding: 35px; color: #aaa; line-height: 1.8;">
                    { if is_en { "Engineered autonomous systems for full-scale Golf Carts, integrating LiDAR, IMU, and Vision Cameras into ROS2 frameworks with MySQL telemetry management." } 
                      else { "實作實體高爾夫球車控制，將 LiDAR、IMU 與視覺相機訊號整合至 ROS2 架構中，並使用 MySQL 進行遙測數據管理與 Linux 環境建置。" } }
                </div>
            </div>
        </div>
    }
}
