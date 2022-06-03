use yew::prelude::*;
use yew_router::prelude::*;
mod components;
use components::Home::Home;
use components::NotFound::NotFound;

#[derive(Clone, Routable, PartialEq)]
enum Routing {
    #[at("/")]
    Home,
    #[at("/editor")]
    Editor,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Routing> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn switch(routes: &Routing) -> Html {
    match routes {
        Routing::Home => html! {<Home />},
        Routing::Editor => html! {<App />},
        Routing::NotFound => html! {<NotFound />},
    }
}

fn main() {
    yew::start_app::<App>();
}
