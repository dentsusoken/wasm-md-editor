use stylist::{css, style};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component(Text)]
pub fn text() -> Html {
    // let split_style = css!(
    //     r#"
    //     display: block;
    //     "#
    // );
    // let split_item = css!(
    //     r#"
    //     display: block;
    //     width: auto;
    //     "#
    // );
    // let split_left_inner = css!(
    //     r#"
    //     position: inherit;
    //     width: auto;
    //     "#
    // );

    let value = use_state(|| String::from(""));
    let on_input = {
        let value = value.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            value.set(input.value());
        })
    };

    html! {
        <>
        <div>
            <div>
                <textarea rows="140" cols="100" value={value.to_string()} oninput={on_input} />
            </div>
            <div>
                {value.to_string()}
            </div>
        </div>
        </>
    }
}
