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
    
    // 定義外層導覽列樣式
    let nav_container_style = "display: flex; justify-content: center; align-items: center; gap: clamp(10px, 3vw, 40px); padding: 20px; background: rgba(0,0,0,0.8); backdrop-filter: blur(15px); border-bottom: 1px solid rgba(255,255,255,0.05); position: fixed; width: 100%; top: 0; z-index: 1000; flex-wrap: wrap;";
    
    // 定義文字樣式 (套用在 span 上)
    let link_text_style = "color: #fff; text-decoration: none; font-weight: 600; opacity: 0.8; font-size: 0.9rem; letter-spacing: 2px; cursor: pointer;";
    
    // 按鈕樣式
    let btn_style = "background: rgba(255,255,255,0.1); border: 1px solid rgba(255,255,255,0.2); color: #fff; padding: 6px 12px; border-radius: 4px; cursor: pointer; font-size: 0.8rem; font-weight: bold; margin-left: auto;";

    html! {
        <nav style={nav_container_style}>
            
            // 修正這裡：加入中英文判斷
            <Link<Route> to={Route::Home}>
                <span style={link_text_style}>
                    { if is_en { "/HOME" } else { "/首頁" } }
                </span>
            </Link<Route>>
            
            <Link<Route> to={Route::Projects}>
                <span style={link_text_style}>
                    { if is_en { "/PROJECTS" } else { "/深度專案" } }
                </span>
            </Link<Route>>
            
            <Link<Route> to={Route::Cv}>
                <span style={link_text_style}>
                    { if is_en { "/EXPERIENCE" } else { "/職涯故事" } }
                </span>
            </Link<Route>>
            
            <button onclick={props.on_toggle.clone()} style={btn_style}>
                { "EN / 中文" }
            </button>
            
        </nav>
    }
}
