use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod pages;
use components::editor::Text;
use pages::not_found::NotFound;
use pages::top::Top;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
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
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Top />},
        Route::Editor => html! {<Text/>},
        Route::NotFound => html! {<NotFound />},
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
