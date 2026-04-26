use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::nav::Nav;
use components::home::Home;
use components::projects::Projects;
use components::cv::Cv;
use components::certificates::Certificates;
use components::contact::Contact;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")] Home,
    #[at("/projects")] Projects,
    #[at("/cv")] Cv,
    #[at("/certs")] Certificates,
    #[at("/contact")] Contact,
    #[not_found] #[at("/404")] NotFound,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Language { En, Zh }

fn switch(routes: Route, lang: Language) -> Html {
    match routes {
        Route::Home => html! { <Home {lang} /> },
        Route::Projects => html! { <Projects {lang} /> },
        Route::Cv => html! { <Cv {lang} /> },
        Route::Certificates => html! { <Certificates {lang} /> },
        Route::Contact => html! { <Contact {lang} /> },
        Route::NotFound => html! { <Home {lang} /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let lang = use_state(|| Language::En);

    // 修復：use_callback 確保 Callback 物件在 lang 不變時保持同一個引用，
    // 避免 Nav 因為收到新 Callback 而不必要地重新渲染。
    let on_toggle = use_callback(lang.clone(), |_: MouseEvent, lang| {
        lang.set(if **lang == Language::En { Language::Zh } else { Language::En });
    });

    // 修復：Switch 的 render prop 記憶化，lang 不變時 Switch 不會重新掛載子頁面。
    let switch_fn = use_callback(*lang, |route: Route, lang| switch(route, *lang));

    html! {
        <HashRouter>
            <Nav lang={*lang} on_toggle={on_toggle} />
            <main style="padding-top: 80px;">
                <Switch<Route> render={switch_fn} />
            </main>
        </HashRouter>
    }
}

fn main() { yew::Renderer::<App>::new().render(); }