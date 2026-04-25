use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::nav::Nav;
use components::home::Home;
use components::projects::Projects;
use components::cv::Cv;
use components::certificates::Certificates; // 新增這行

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")] Home,
    #[at("/projects")] Projects,
    #[at("/cv")] Cv,
    #[at("/certs")] Certificates, // 新增這行
    #[not_found] #[at("/404")] NotFound,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Language { En, Zh }

fn switch(routes: Route, lang: Language) -> Html {
    let content = match routes {
        Route::Home => html! { <Home {lang} /> },
        Route::Projects => html! { <Projects {lang} /> },
        Route::Cv => html! { <Cv {lang} /> },
        Route::Certificates => html! { <Certificates {lang} /> }, // 新增這行
        Route::NotFound => html! { <Home {lang} /> },
    };
    html! { <div class="page-transition">{ content }</div> }
}

#[function_component(App)]
fn app() -> Html {
    let lang = use_state(|| Language::En);
    let on_toggle = {
        let lang = lang.clone();
        Callback::from(move |_| {
            lang.set(if *lang == Language::En { Language::Zh } else { Language::En });
        })
    };

    html! {
        <HashRouter>
            <Nav lang={*lang} on_toggle={on_toggle} />
            <main style="padding-top: 80px;">
                <Switch<Route> render={move |r| switch(r, *lang)} />
            </main>
        </HashRouter>
    }
}

fn main() { yew::Renderer::<App>::new().render(); }
