use yew::prelude::*;
use yew_router::prelude::*;
mod components;
use components::Home::Home;
use components::NotFound::NotFound;
use components::Text::Text;
#[derive(Clone, Routable, PartialEq)]
enum Routing {
    #[at("/")]
    Home,
    #[at("/editor")]
    Text,
    #[not_found]
    #[at("/404")]
    NotFound,
}
pub enum Msg {
    SetInput(String),
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
        Routing::Text => html! {<Text  />},
        Routing::NotFound => html! {<NotFound />},
    }
}

fn main() {
    yew::start_app::<App>();
}
