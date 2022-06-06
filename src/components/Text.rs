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
    let value = use_state(|| "init".to_string());
    let on_input = {
        let value = value.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            value.set(input.value());
        })
    };

    // let Props { input, on_change } = props.clone();
    // let oninput = Callback::from(move |input_event: InputEvent| {
    //     on_change.emit(get_value_from_input_event(input_event))
    // });
    html! {
        <textarea value={value.to_string()} oninput={on_input} />
    }
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    target.value()
}
