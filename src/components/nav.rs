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

    let nav_bg = "display: flex; justify-content: space-between; align-items: center; padding: 15px 30px; background: rgba(3,3,3,0.85); backdrop-filter: blur(20px); border-bottom: 1px solid rgba(255,255,255,0.05); position: fixed; width: 100%; top: 0; z-index: 1000;";
    let link_css = "color: #fff; text-decoration: none; font-weight: 700; font-size: 0.8rem; letter-spacing: 2px; opacity: 0.7; transition: 0.3s; cursor: pointer;";
    
    let mobile_menu_css = if *menu_open {
        "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(3, 3, 3, 0.98); display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 35px; z-index: 999; transition: 0.4s; opacity: 1; pointer-events: all;"
    } else {
        "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(3, 3, 3, 0.98); display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 35px; z-index: 999; transition: 0.4s; opacity: 0; pointer-events: none;"
    };

    html! {
        <>
            <nav style={nav_bg}>
                <Link<Route> to={Route::Home}><span style="font-family: 'JetBrains Mono'; font-weight: 900; color: var(--primary);">{ "SJ_LIN" }</span></Link<Route>>

                <div class="desktop-nav">
                    <Link<Route> to={Route::Home}><span style={link_css}>{ if is_en { "/HOME" } else { "/首頁" } }</span></Link<Route>>
                    <Link<Route> to={Route::Projects}><span style={link_css}>{ if is_en { "/PROJECTS" } else { "/專案" } }</span></Link<Route>>
                    <Link<Route> to={Route::Cv}><span style={link_css}>{ if is_en { "/CV" } else { "/歷程" } }</span></Link<Route>>
                    <Link<Route> to={Route::Certificates}><span style={link_css}>{ if is_en { "/CREDENTIALS" } else { "/認證" } }</span></Link<Route>>
                    <Link<Route> to={Route::Contact}><span style={link_css}>{ if is_en { "/CONTACT" } else { "/聯絡" } }</span></Link<Route>>
                    <button onclick={props.on_toggle.clone()} style="background: var(--primary); border: none; color: #fff; padding: 4px 12px; border-radius: 4px; font-weight: 900; cursor: pointer;">{ if is_en { "ZH" } else { "EN" } }</button>
                </div>

                <button class="mobile-btn" onclick={toggle_menu}><i class={if *menu_open { "fas fa-times" } else { "fas fa-bars" }}></i></button>
            </nav>

            <div style={mobile_menu_css}>
                <Link<Route> to={Route::Home}><div onclick={close_menu.clone()} style="font-size: 2rem; font-weight: 900; color: #fff; letter-spacing: 4px;">{ if is_en { "HOME" } else { "首頁" } }</div></Link<Route>>
                <Link<Route> to={Route::Projects}><div onclick={close_menu.clone()} style="font-size: 2rem; font-weight: 900; color: #fff; letter-spacing: 4px;">{ if is_en { "PROJECTS" } else { "專案" } }</div></Link<Route>>
                <Link<Route> to={Route::Cv}><div onclick={close_menu.clone()} style="font-size: 2rem; font-weight: 900; color: #fff; letter-spacing: 4px;">{ if is_en { "CV" } else { "歷程" } }</div></Link<Route>>
                <Link<Route> to={Route::Certificates}><div onclick={close_menu.clone()} style="font-size: 2rem; font-weight: 900; color: #fff; letter-spacing: 4px;">{ if is_en { "CREDENTIALS" } else { "認證" } }</div></Link<Route>>
                <Link<Route> to={Route::Contact}><div onclick={close_menu.clone()} style="font-size: 2rem; font-weight: 900; color: #fff; letter-spacing: 4px;">{ if is_en { "CONTACT" } else { "聯絡" } }</div></Link<Route>>
                <button onclick={props.on_toggle.clone()} style="margin-top: 20px; background: var(--primary); border: none; color: #fff; padding: 10px 40px; border-radius: 30px; font-weight: 900;">{ if is_en { "ZH" } else { "EN" } }</button>
            </div>
        </>
    }
}
