use yew::prelude::*;
use crate::Language;

#[function_component(Contact)]
pub fn contact(props: &super::home::PageProps) -> Html {
    let is_en = props.lang == Language::En;
    let card_style = "padding: 40px; text-align: center; display: flex; flex-direction: column; gap: 25px; max-width: 600px; margin: 0 auto;";
    let link_style = "color: var(--primary); text-decoration: none; font-family: 'JetBrains Mono'; font-weight: bold; font-size: 1.2rem; transition: 0.3s;";

    html! {
        <div class="container" style="padding: 100px 24px;">
            <h2>{ if is_en { "GET_IN_TOUCH" } else { "聯絡與社群" } }</h2>
            <div class="glass" style={card_style}>
                <div style="font-family: 'JetBrains Mono'; color: var(--accent); margin-bottom: 10px;">{ "> 正在連線至架構師終端..." }</div>
                
                <a href="mailto:rennn223@gmail.com" style={link_style}>{ "[ EMAIL: rennn223@gmail.com ]" }</a>
                <a href="https://github.com/rennn0223" target="_blank" style={link_style}>{ "[ GITHUB: rennn0223 ]" }</a>
                <a href="tel:+8869XXXXXXXX" style={link_style}>{ "[ PHONE: +886-9XX-XXX-XXX ]" }</a>
                <a href="https://blinq.me/u/rennn223" target="_blank" style={link_style}>{ "[ BLINQ: DIGITAL_CARD ]" }</a>
                
                <div style="margin-top: 20px; font-size: 0.8rem; color: #666;">{ if is_en { "Available for Digital Twin & AI Infrastructure collaborations." } else { "歡迎關於數位孿生與 AI 基礎設施的開發合作。" } }</div>
            </div>
        </div>
    }
}
