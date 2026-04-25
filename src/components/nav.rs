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

    let nav_bg = "display: flex; justify-content: space-between; align-items: center; padding: 15px 30px; background: rgba(3,3,3,0.8); backdrop-filter: blur(20px); border-bottom: 1px solid rgba(255,255,255,0.05); position: fixed; width: 100%; top: 0; z-index: 1000;";
    let link_css = "color: #fff; text-decoration: none; font-weight: 700; font-size: 0.85rem; letter-spacing: 2px; opacity: 0.7; transition: 0.3s;";
    
    // 手機版全螢幕選單樣式
    let mobile_menu_css = if *menu_open {
        "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(3, 3, 3, 0.98); display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 40px; z-index: 999; transition: all 0.4s ease; opacity: 1; pointer-events: all;"
    } else {
        "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(3, 3, 3, 0.98); display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 40px; z-index: 999; transition: all 0.4s ease; opacity: 0; pointer-events: none;"
    };

    let mobile_link_css = "color: #fff; text-decoration: none; font-weight: 900; font-size: 2rem; letter-spacing: 4px; transition: 0.3s;";

    html! {
        <>
            /* 頂部固定欄 (Header) */
            <nav style={nav_bg}>
                /* 左側 Logo (可點擊回首頁) */
                <Link<Route> to={Route::Home} classes={classes!("nav-logo")}>
                    <span style="font-family: 'JetBrains Mono', monospace; font-weight: 900; color: var(--primary); font-size: 1.2rem; cursor: pointer;">{ "SJ_LIN" }</span>
                </Link<Route>>

                /* 電腦版選單 (Desktop) */
                <div class="desktop-nav">
                    <Link<Route> to={Route::Home}><span style={link_css}>{ if is_en { "/HOME" } else { "/首頁" } }</span></Link<Route>>
                    <Link<Route> to={Route::Projects}><span style={link_css}>{ if is_en { "/PROJECTS" } else { "/專案" } }</span></Link<Route>>
                    <Link<Route> to={Route::Cv}><span style={link_css}>{ if is_en { "/CV" } else { "/歷程" } }</span></Link<Route>>
                    <Link<Route> to={Route::Certificates}><span style={link_css}>{ if is_en { "/CREDENTIALS" } else { "/專業認證" } }</span></Link<Route>>
                    
                    <button onclick={props.on_toggle.clone()} style="background: var(--primary); color: #fff; border: none; padding: 6px 16px; border-radius: 4px; font-weight: 900; font-size: 0.75rem; cursor: pointer; transition: 0.3s;">
                        { if is_en { "ZH" } else { "EN" } }
                    </button>
                </div>

                /* 手機版漢堡按鈕 (Mobile Toggle) */
                <button class="mobile-btn" onclick={toggle_menu}>
                    <i class={if *menu_open { "fas fa-times" } else { "fas fa-bars" }}></i>
                </button>
            </nav>

            /* 手機版全螢幕遮罩選單 (Mobile Fullscreen Overlay) */
            <div style={mobile_menu_css}>
                <Link<Route> to={Route::Home}><div onclick={close_menu.clone()} style={mobile_link_css}>{ if is_en { "HOME" } else { "首頁" } }</div></Link<Route>>
                <Link<Route> to={Route::Projects}><div onclick={close_menu.clone()} style={mobile_link_css}>{ if is_en { "PROJECTS" } else { "專案" } }</div></Link<Route>>
                <Link<Route> to={Route::Cv}><div onclick={close_menu.clone()} style={mobile_link_css}>{ if is_en { "CV" } else { "歷程" } }</div></Link<Route>>
                <Link<Route> to={Route::Certificates}><div onclick={close_menu.clone()} style={mobile_link_css}>{ if is_en { "CREDENTIALS" } else { "專業認證" } }</div></Link<Route>>
                
                <button onclick={props.on_toggle.clone()} style="margin-top: 20px; background: transparent; border: 2px solid var(--primary); color: var(--primary); padding: 10px 40px; border-radius: 30px; font-weight: 900; font-size: 1.2rem; cursor: pointer;">
                    { if is_en { "SWITCH TO 中文" } else { "切換至 ENGLISH" } }
                </button>
            </div>
        </>
    }
}
