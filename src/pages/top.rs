use crate::{components::home::Home, Route};
use stylist::style;
use yew::prelude::*;
use yew_router::prelude::*;

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
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Editor));

    html! {
        <>
            <div class={classes!(container)}>
                <Home />
                <button class={classes!(button)} {onclick}>{"Start"}</ button>
            </div>
        </>
    }
}
