use yew::prelude::*;
use yew_router::prelude::*;
use crate::{Route, Language};

#[derive(Properties, PartialEq)]
pub struct NavProps { pub lang: Language, pub on_toggle: Callback<MouseEvent> }

#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    let is_en = props.lang == Language::En;
    
    // 狀態管理：控制手機版選單的開關
    let menu_open = use_state(|| false);

    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };

    // 點擊連結後自動關閉選單
    let close_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(false))
    };

    let nav_css = "display: flex; justify-content: space-between; align-items: center; padding: 15px 30px; background: rgba(3,3,3,0.85); backdrop-filter: blur(20px); border-bottom: 1px solid rgba(255,255,255,0.05); position: fixed; width: 100%; top: 0; z-index: 1000;";
    let link_css = "color: #fff; text-decoration: none; font-weight: 700; font-size: 0.85rem; letter-spacing: 2px; opacity: 0.7; transition: 0.3s;";
    
    // 手機版全螢幕選單樣式
    let mobile_menu_css = if *menu_open {
        "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(3, 3, 3, 0.98); display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 30px; z-index: 999; transition: all 0.4s ease; opacity: 1; pointer-events: all;"
    } else {
        "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(3, 3, 3, 0.98); display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 30px; z-index: 999; transition: all 0.4s ease; opacity: 0; pointer-events: none;"
    };

    let mobile_link_css = "color: #fff; text-decoration: none; font-weight: 900; font-size: 2rem; letter-spacing: 4px; transition: 0.3s;";

    // 超關鍵：Logo 變色 CSS 魔法 (Brightness 0 變成全黑，再 Invert 1 變成全白)
    let logo_filter = "height: 25px; margin: 0 8px; filter: brightness(0) invert(1);";

    html! {
        <>
            <nav style={nav_css}>
                /* 左側：移除名字，換成聯合品牌 Logo */
                <Link<Route> to={Route::Home}>
                    <div style="display: flex; align-items: center; cursor: pointer;">
                        <img src="assets/nvidia.png" alt="NVIDIA" style={logo_filter} />
                        <span style="color: rgba(255,255,255,0.2); font-weight: 900;">{ "x" }</span>
                        <img src="assets/nchu.png" alt="NCHU" style={logo_filter} />
                        <span style="color: rgba(255,255,255,0.2); font-weight: 900;">{ "x" }</span>
                        <img src="assets/msi.png" alt="MSI" style={logo_filter} />
                    </div>
                </Link<Route>>

                /* 電腦版導覽列 */
                <div class="desktop-nav">
                    <Link<Route> to={Route::Home}><span style={link_css}>{ if is_en { "/HOME" } else { "/首頁" } }</span></Link<Route>>
                    <Link<Route> to={Route::Projects}><span style={link_css}>{ if is_en { "/PROJECTS" } else { "/專案" } }</span></Link<Route>>
                    <Link<Route> to={Route::Cv}><span style={link_css}>{ if is_en { "/CV" } else { "/歷程" } }</span></Link<Route>>
                    <Link<Route> to={Route::Contact}><span style={link_css}>{ if is_en { "/CONTACT" } else { "/聯絡" } }</span></Link<Route>>
                    <button onclick={props.on_toggle.clone()} style="background: var(--primary); color: #fff; border: none; padding: 4px 12px; border-radius: 4px; font-weight: 900; cursor: pointer;">
                        { if is_en { "ZH" } else { "EN" } }
                    </button>
                </div>

                /* 手機版選單按鈕 */
                <button class="mobile-btn" onclick={toggle_menu}>
                    <i class={if *menu_open { "fas fa-times" } else { "fas fa-bars" }}></i>
                </button>
            </nav>

            /* 手機版全螢幕選單 (Apple/NVIDIA 風格) */
            <div style={mobile_menu_css}>
                /* 選單頂部：大頭貼與聯合 Logo */
                <div style="display: flex; flex-direction: column; align-items: center; gap: 12px; margin-bottom: 10px;">
                    <img src="assets/profile.jpeg" alt="Profile" style="width: 80px; height: 80px; border-radius: 50%; border: 2px solid var(--primary); box-shadow: 0 0 15px rgba(59, 130, 246, 0.5); object-fit: cover;" />
                    <div style="display: flex; align-items: center; justify-content: center; width: 100%;">
                        <img src="assets/nvidia.png" alt="NVIDIA" style="height: 18px; filter: brightness(0) invert(1);" />
                        <img src="assets/nchu.png" alt="NCHU" style="height: 18px; filter: brightness(0) invert(1);" />
                        <img src="assets/msi.png" alt="MSI" style="height: 18px; filter: brightness(0) invert(1);" />
                    </div>
                </div>

                <Link<Route> to={Route::Home}><div onclick={close_menu.clone()} style={mobile_link_css}>{ if is_en { "HOME" } else { "首頁" } }</div></Link<Route>>
                <Link<Route> to={Route::Projects}><div onclick={close_menu.clone()} style={mobile_link_css}>{ if is_en { "PROJECTS" } else { "專案" } }</div></Link<Route>>
                <Link<Route> to={Route::Cv}><div onclick={close_menu.clone()} style={mobile_link_css}>{ if is_en { "CV" } else { "歷程" } }</div></Link<Route>>
                <Link<Route> to={Route::Contact}><div onclick={close_menu.clone()} style={mobile_link_css}>{ if is_en { "CONTACT" } else { "聯絡" } }</div></Link<Route>>
                
                <button onclick={props.on_toggle.clone()} style="margin-top: 20px; background: transparent; border: 2px solid var(--primary); color: var(--primary); padding: 10px 40px; border-radius: 30px; font-weight: 900;">
                    { if is_en { "SWITCH TO 中文" } else { "切換至 ENGLISH" } }
                </button>
            </div>
        </>
    }
}
