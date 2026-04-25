use yew::prelude::*;
use yew_router::prelude::*;
use crate::{Route, Language};

#[derive(Properties, PartialEq)]
pub struct NavProps { pub lang: Language, pub on_toggle: Callback<MouseEvent> }

#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    let is_en = props.lang == Language::En;
    let menu_open = use_state(|| false);
    let toggle_menu = { let menu_open = menu_open.clone(); Callback::from(move |_| menu_open.set(!*menu_open)) };
    let close_menu = { let menu_open = menu_open.clone(); Callback::from(move |_| menu_open.set(false)) };

    // 導覽列底色與佈局
    let nav_bg = "display: flex; justify-content: space-between; align-items: center; padding: 15px clamp(15px, 4vw, 30px); background: rgba(3,3,3,0.85); backdrop-filter: blur(20px); border-bottom: 1px solid rgba(255,255,255,0.05); position: fixed; width: 100%; top: 0; z-index: 1000;";
    let link_css = "color: #fff; text-decoration: none; font-weight: 700; font-size: 0.8rem; letter-spacing: 2px; opacity: 0.7; transition: 0.3s; cursor: pointer;";
    
    let mobile_menu_css = if *menu_open {
        "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(3, 3, 3, 0.98); display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 30px; z-index: 999; transition: 0.4s; opacity: 1; pointer-events: all;"
    } else {
        "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(3, 3, 3, 0.98); display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 30px; z-index: 999; transition: 0.4s; opacity: 0; pointer-events: none;"
    };

    let mobile_link_css = "font-size: clamp(1.5rem, 6vw, 2rem); font-weight: 900; color: #fff; letter-spacing: 4px; transition: 0.3s; text-decoration: none;";

    // -------------------------------------------------------------
    // 終極修復：Logo 尺寸與顏色魔法
    // 1. height: clamp(14px, 3.5vw, 22px) -> 確保手機上縮小，電腦上剛好
    // 2. filter: brightness(0) invert(1) -> 把任何顏色的圖強迫變成純白色
    // -------------------------------------------------------------
    let logo_container = "display: flex; align-items: center; gap: clamp(4px, 1.5vw, 10px); z-index: 1001;";
    let logo_style = "height: clamp(14px, 3.5vw, 22px); width: auto; object-fit: contain; filter: brightness(0) invert(1);";
    let x_style = "color: rgba(255,255,255,0.3); font-weight: 900; font-size: clamp(0.7rem, 2vw, 1rem);";

    html! {
        <>
            <nav style={nav_bg}>
                /* 左側：修正後的 3 張聯合 Logo */
                <Link<Route> to={Route::Home} style="text-decoration: none; flex-shrink: 1;">
                    <div style={logo_container}>
                        <img src="assets/nvidia.png" alt="NVIDIA" style={logo_style} />
                        <span style={x_style}>{ "x" }</span>
                        <img src="assets/nchu.png" alt="NCHU" style={logo_style} />
                        <span style={x_style}>{ "x" }</span>
                        <img src="assets/msi.png" alt="MSI" style={logo_style} />
                    </div>
                </Link<Route>>

                <div class="desktop-nav">
                    <Link<Route> to={Route::Home}><span style={link_css}>{ if is_en { "/HOME" } else { "/首頁" } }</span></Link<Route>>
                    <Link<Route> to={Route::Projects}><span style={link_css}>{ if is_en { "/PROJECTS" } else { "/專案" } }</span></Link<Route>>
                    <Link<Route> to={Route::Cv}><span style={link_css}>{ if is_en { "/CV" } else { "/歷程" } }</span></Link<Route>>
                    <Link<Route> to={Route::Certificates}><span style={link_css}>{ if is_en { "/CERTIFICATES" } else { "/證照" } }</span></Link<Route>>
                    <Link<Route> to={Route::Contact}><span style={link_css}>{ if is_en { "/CONTACT" } else { "/聯絡" } }</span></Link<Route>>
                    <button onclick={props.on_toggle.clone()} style="background: var(--primary); border: none; color: #fff; padding: 4px 12px; border-radius: 4px; font-weight: 900; cursor: pointer; transition: 0.3s;">{ if is_en { "ZH" } else { "EN" } }</button>
                </div>

                /* 確保漢堡按鈕 z-index 最高，絕對不會被擋住 */
                <button class="mobile-btn" style="position: relative; z-index: 1005; padding: 5px;" onclick={toggle_menu}>
                    <i class={if *menu_open { "fas fa-times" } else { "fas fa-bars" }}></i>
                </button>
            </nav>

            <div style={mobile_menu_css}>
                /* 手機版選單：大頭貼與 3 張聯合 Logo */
                <div style="display: flex; flex-direction: column; align-items: center; gap: 15px; margin-bottom: 10px;">
                    <img src="assets/profile.jpeg" alt="Profile Avatar" style="width: 90px; height: 90px; border-radius: 50%; border: 2px solid var(--primary); box-shadow: 0 0 15px rgba(59, 130, 246, 0.5); object-fit: cover;" />
                    
                    <div style="display: flex; align-items: center; justify-content: center; gap: 8px;">
                        <img src="assets/nvidia.png" alt="NVIDIA" style="height: 16px; filter: brightness(0) invert(1);" />
                        <span style="color: rgba(255,255,255,0.3); font-size: 0.8rem;">{ "x" }</span>
                        <img src="assets/nchu.png" alt="NCHU" style="height: 16px; filter: brightness(0) invert(1);" />
                        <span style="color: rgba(255,255,255,0.3); font-size: 0.8rem;">{ "x" }</span>
                        <img src="assets/msi.png" alt="MSI" style="height: 16px; filter: brightness(0) invert(1);" />
                    </div>
                </div>

                <Link<Route> to={Route::Home}><div onclick={close_menu.clone()} style={mobile_link_css}>{ if is_en { "HOME" } else { "首頁" } }</div></Link<Route>>
                <Link<Route> to={Route::Projects}><div onclick={close_menu.clone()} style={mobile_link_css}>{ if is_en { "PROJECTS" } else { "專案" } }</div></Link<Route>>
                <Link<Route> to={Route::Cv}><div onclick={close_menu.clone()} style={mobile_link_css}>{ if is_en { "CV" } else { "歷程" } }</div></Link<Route>>
                <Link<Route> to={Route::Certificates}><div onclick={close_menu.clone()} style={mobile_link_css}>{ if is_en { "CERTIFICATES" } else { "證照" } }</div></Link<Route>>
                <Link<Route> to={Route::Contact}><div onclick={close_menu.clone()} style={mobile_link_css}>{ if is_en { "CONTACT" } else { "聯絡" } }</div></Link<Route>>
                
                <button onclick={props.on_toggle.clone()} style="margin-top: 10px; background: transparent; border: 2px solid var(--primary); color: var(--primary); padding: 10px 40px; border-radius: 30px; font-weight: 900; cursor: pointer;">
                    { if is_en { "SWITCH TO 中文" } else { "切換至 ENGLISH" } }
                </button>
            </div>
        </>
    }
}
