use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::nav::Nav;
use components::home::Home;
use components::projects::Projects;
use components::cv::Cv;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/cv")]
    Cv,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Language { En, Zh }

fn switch(routes: Route, lang: Language) -> Html {
    match routes {
        Route::Home => html! { <Home {lang} /> },
        Route::Projects => html! { <Projects {lang} /> },
        Route::Cv => html! { <Cv {lang} /> },
        Route::NotFound => html! { <h1 style="padding-top:150px; text-align:center;">{ "404 - Not Found" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let lang = use_state(|| Language::En);
    let on_toggle_lang = {
        let lang = lang.clone();
        Callback::from(move |_| {
            lang.set(if *lang == Language::En { Language::Zh } else { Language::En });
        })
    };

    html! {
        <BrowserRouter>
            <Nav lang={*lang} on_toggle={on_toggle_lang} />
            <main class={if *lang == Language::En { "lang-en" } else { "lang-zh" }}>
                <Switch<Route> render={move |r| switch(r, *lang)} />
            </main>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
