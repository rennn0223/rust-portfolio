use yew::prelude::*;
use crate::Language;

#[derive(Properties, PartialEq)]
pub struct PageProps { pub lang: Language }

#[function_component(Cv)]
pub fn cv(props: &PageProps) -> Html {
    let is_en = props.lang == Language::En;
    html! {
        <div class="container" style="padding-top: 120px; padding-bottom: 80px;">
            <h1 class="section-title">{ if is_en { "My Engineering Journey" } else { "工程師進化軌跡" } }</h1>
            <div style="position: relative; padding-left: 25px; border-left: 2px solid rgba(255,255,255,0.1); margin-left: 10px;">
                
                // Timeline Item: Infrastructure
                <div style="margin-bottom: 45px; position: relative;">
                    <span style="font-family: 'JetBrains Mono', monospace; font-size: 0.8rem; font-weight: bold; color: #10b981; background: rgba(16, 185, 129, 0.1); padding: 4px 10px; border-radius: 4px;">{ "2026.03.30 — PRESENT" }</span>
                    <h3 style="font-size: 1.3rem; font-weight: 900; margin: 10px 0;">{ if is_en { "Infrastructure & LLM Integration" } else { "伺服器優化與 LLM 應用導入" } }</h3>
                    <p style="color: #aaa; font-size: 0.95rem;">{ if is_en { "Managing MGX Server and NemoClaw on DGX Spark to simplify Omniverse workflows." } else { "負責 MGX Server 升級與環境建置。並部署 NemoClaw，透過 LLM 解決新手技術痛點。" } }</p>
                </div>

                // Timeline Item: GTC
                <div style="margin-bottom: 45px; position: relative;">
                    <span style="font-family: 'JetBrains Mono', monospace; font-size: 0.8rem; font-weight: bold; color: #10b981; background: rgba(16, 185, 129, 0.1); padding: 4px 10px; border-radius: 4px;">{ "2026.03" }</span>
                    <h3 style="font-size: 1.3rem; font-weight: 900; margin: 10px 0;">{ if is_en { "GTC 2026 Exhibitor & BAIR Visit" } else { "GTC 2026 參展與 UC Berkeley 交流" } }</h3>
                    <p style="color: #aaa; font-size: 0.95rem;">{ if is_en { "Exhibited at MSI DGX Spark booth, California. Received Jensen Huang's signature. Invited to UC Berkeley BAIR for discussions." } else { "赴美加州於微星攤位展出數位孿生專案，榮獲黃仁勳親筆簽名。受邀訪問柏克萊大學 BAIR 中心深度交流。" } }</p>
                </div>

                // Timeline Item: TA
                <div style="margin-bottom: 45px; position: relative;">
                    <span style="font-family: 'JetBrains Mono', monospace; font-size: 0.8rem; font-weight: bold; color: #10b981; background: rgba(16, 185, 129, 0.1); padding: 4px 10px; border-radius: 4px;">{ "2026.02 — PRESENT" }</span>
                    <h3 style="font-size: 1.3rem; font-weight: 900; margin: 10px 0;">{ if is_en { "Teaching Assistant: Electronics (A)" } else { "電子學 (A) 教學助理" } }</h3>
                    <p style="color: #aaa; font-size: 0.95rem;">{ if is_en { "Mentoring undergrads, strengthening technical communication and hardware mastery." } else { "擔任大學部電子學 TA，輔導解答學理，提升技術溝通與指導能力。" } }</p>
                </div>

                // Timeline Item: MSI Residency
                <div style="margin-bottom: 45px; position: relative;">
                    <span style="font-family: 'JetBrains Mono', monospace; font-size: 0.8rem; font-weight: bold; color: #10b981; background: rgba(16, 185, 129, 0.1); padding: 4px 10px; border-radius: 4px;">{ "2026.01 — 2026.02" }</span>
                    <h3 style="font-size: 1.3rem; font-weight: 900; margin: 10px 0;">{ if is_en { "MSI HQ Collaborative Residency" } else { "微星總部駐點協同開發" } }</h3>
                    <p style="color: #aaa; font-size: 0.95rem;">{ if is_en { "Collaborated with Senior Manager Hung Shih-Che and RD Lai Yen-Cheng on Digital Twin and NVIDIA Alpamayo R&D." } else { "進駐微星中和總部，與資深經理洪士哲及 RD 賴彥成協同開發 GTC 2026 數位孿生與 NVIDIA Alpamayo 應用。" } }</p>
                </div>

                // Timeline Item: Education
                <div style="margin-bottom: 45px; position: relative;">
                    <span style="font-family: 'JetBrains Mono', monospace; font-size: 0.8rem; font-weight: bold; color: #10b981; background: rgba(16, 185, 129, 0.1); padding: 4px 10px; border-radius: 4px;">{ "EDUCATION" }</span>
                    <h3 style="font-size: 1.3rem; font-weight: 900; margin: 10px 0;">{ if is_en { "UC Cincinnati & National Chung Hsing University" } else { "美國辛辛那提大學與中興大學" } }</h3>
                    <p style="color: #aaa; font-size: 0.95rem;">
                        { if is_en { "Master in Mechanical Engineering (Dual Degree Expected 2026) | NCHU Master GPA 3.76" } 
                          else { "機械工程學系雙聯碩士 (預定 2026 秋) | 中興碩士 GPA 3.76" } }
                    </p>
                </div>

            </div>
        </div>
    }
}
