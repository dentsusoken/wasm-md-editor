use crate::{components::home::Home, Routing};
use stylist::style;
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

#[function_component(Top)]
pub fn top() -> Html {
    let container = style!(
        r#"
        display: flex;
        flex-direction: column;
        align-items: center;
        "#
    )
    .expect("Failed to styled.");
    let button = style!(
        r#"
        color: #ffffff;
        width: 200px;
        padding: 10px;
        background-color: #1976d2;
        box-shadow: 0 3px 5px rgba(0, 0, 0, .3);
        -webkit-box-shadow: 0 3px 5px rgba(0, 0, 0, .3);
        :hover {
            background: #115293;
            margin-top: 3px;
        }
        "#
    )
    .expect("Failed to styled.");
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Routing::Editor));

    html! {
        <>
            <div class={container}>
                <Home />
                <button class={button} {onclick}>{"Start"}</ button>
            </div>
        </>
    }
}
