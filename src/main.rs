use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod pages;
use components::editor::Text;
use components::not_found::NotFound;
use pages::top::Top;

#[derive(Clone, Routable, PartialEq)]
pub enum Routing {
    #[at("/")]
    Home,
    #[at("/editor")]
    Editor,
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
        Routing::Home => html! {
            <Top />
        },
        Routing::Editor => html! {
            <Text  />

        },
        Routing::NotFound => html! {<NotFound />},
    }
}

fn main() {
    yew::start_app::<App>();
}
