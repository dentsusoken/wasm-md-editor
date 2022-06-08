use super::markdown::Markdown;
use css_in_rust::style::Style;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component(Text)]
pub fn text() -> Html {
    let style = Style::create(
        "Text",
        r#"
    background-color: black;
    "#,
    )
    .unwrap();

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
                <Markdown markdwon_data={value.to_string()} />
                {value.to_string()}
            </div>
        </div>
        </>
    }
}
