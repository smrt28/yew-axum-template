use yew::Html;
use yew::html;
use yew_router::Routable;
use yew::function_component;
use yew_router::BrowserRouter;
use yew_router::Switch;

mod components;

use crate::components::home;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/home")]  // Add this line
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::NotFound => html! { <div class={"html-404"}><a href="home"><h1>{ "* 404 *" }</h1></a></div> },
        Route::Home => html! { <home::Home /> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}