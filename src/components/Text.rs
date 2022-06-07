use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlTextAreaElement;
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub input: String,
    pub on_change: Callback<String>,
}

#[function_component(Text)]
pub fn text() -> Html {
    let value = use_state(|| String::from(""));
    let on_input = {
        let value = value.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            value.set(input.value());
        })
    };

    html! {
        <textarea rows="140" cols="100" value={value.to_string()} oninput={on_input} />
    }
}
