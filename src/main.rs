use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::nav::Nav;
use components::home::Home;
use components::projects::Projects;
use components::cv::Cv;

// 路由定義不變
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
        Route::NotFound => html! { <Home {lang} /> }, // 如果真的找不到，直接導回 Home 顯得專業
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
        <HashRouter> // <-- 這裡改成 HashRouter 是關鍵
            <Nav lang={*lang} on_toggle={on_toggle_lang} />
            <main class={if *lang == Language::En { "lang-en" } else { "lang-zh" }}>
                <Switch<Route> render={move |r| switch(r, *lang)} />
            </main>
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
