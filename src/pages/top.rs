use crate::{components::home::Home, Routing};
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

#[function_component(Top)]
pub fn top() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Routing::Editor));
    html! {
        <>
        <Home />
        <button class="button" {onclick}>{"Start"}</ button>
        </>
    }
}
