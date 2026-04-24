use yew::prelude::*;
use yew_router::prelude::*;
use crate::{Route, Language};

#[derive(Properties, PartialEq)]
pub struct NavProps {
    pub lang: Language,
    pub on_toggle: Callback<MouseEvent>,
}

#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    let is_en = props.lang == Language::En;
    let nav_style = "display: flex; justify-content: center; align-items: center; gap: clamp(15px, 4vw, 50px); padding: 15px 30px; background: rgba(5,5,5,0.7); backdrop-filter: blur(20px); border-bottom: 1px solid rgba(255,255,255,0.1); position: fixed; width: 100%; top: 0; z-index: 2000;";
    let link_style = "color: #fff; text-decoration: none; font-weight: 700; font-size: 0.85rem; letter-spacing: 2px; opacity: 0.7; transition: 0.3s; cursor: pointer;";

    html! {
        <nav style={nav_style}>
            <Link<Route> to={Route::Home}><span class="nav-link" style={link_style}>{ if is_en { "HOME" } else { "首頁" } }</span></Link<Route>>
            <Link<Route> to={Route::Projects}><span class="nav-link" style={link_style}>{ if is_en { "TECH" } else { "技術專案" } }</span></Link<Route>>
            <Link<Route> to={Route::Cv}><span class="nav-link" style={link_style}>{ if is_en { "JOURNEY" } else { "歷程" } }</span></Link<Route>>
            <button onclick={props.on_toggle.clone()} style="background: var(--primary); color: #fff; border: none; padding: 6px 15px; border-radius: 20px; font-weight: 900; font-size: 0.7rem; cursor: pointer; margin-left: auto;">
                { if is_en { "ZH" } else { "EN" } }
            </button>
        </nav>
    }
}
