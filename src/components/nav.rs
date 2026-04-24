use yew::prelude::*;
use yew_router::prelude::*;
use crate::{Route, Language};

#[derive(Properties, PartialEq)]
pub struct NavProps { pub lang: Language, pub on_toggle: Callback<MouseEvent> }

#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    let is_en = props.lang == Language::En;
    let nav_css = "display: flex; justify-content: center; align-items: center; gap: clamp(10px, 4vw, 40px); padding: 15px 30px; background: rgba(3,3,3,0.7); backdrop-filter: blur(20px); border-bottom: 1px solid rgba(255,255,255,0.1); position: fixed; width: 100%; top: 0; z-index: 1001;";
    let link_css = "color: #fff; text-decoration: none; font-weight: 700; font-size: 0.8rem; letter-spacing: 2px; opacity: 0.7; transition: 0.3s; cursor: pointer;";

    html! {
        <nav style={nav_css}>
            <Link<Route> to={Route::Home}><span style={link_css}>{ if is_en { "/HOME" } else { "/首頁" } }</span></Link<Route>>
            <Link<Route> to={Route::Projects}><span style={link_css}>{ if is_en { "/PROJECTS" } else { "/專案" } }</span></Link<Route>>
            <Link<Route> to={Route::Cv}><span style={link_css}>{ if is_en { "/CV" } else { "/歷程" } }</span></Link<Route>>
            <button onclick={props.on_toggle.clone()} style="margin-left: auto; background: var(--primary); color: #fff; border: none; padding: 6px 15px; border-radius: 4px; font-weight: 900; font-size: 0.7rem; cursor: pointer;">
                { if is_en { "ZH" } else { "EN" } }
            </button>
        </nav>
    }
}
