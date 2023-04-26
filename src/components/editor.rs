use pulldown_cmark::{html, Options, Parser};
use stylist::style;
use stylist::yew::styled_component;
use web_sys::HtmlTextAreaElement;
use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;

// #[function_component(Text)]
#[styled_component(Text)]
pub fn text() -> Html {
    let text = style!(
        r#"
        background-color: #1e2126;
        color: #fff;
        resize: none;
        font-family: inherit;
        width:100%;
        height:100%;
        "#
    )
    .expect("Failed to styled.");

    let container = style!(
        r#"
        margin: 10px;
        "#
    )
    .expect("Failed to styled.");

    let split = style!(
        r#"
        position: absolute;
        width: 50%;
        height: 100%;
        "#
    )
    .expect("Failed to styled.");
    let split_input = style!(
        r#"
        left: 0;
        "#
    )
    .expect("Failed to styled.");

    let split_output = style!(
        r#"
        right: 0;
        margin: -25px;
        "#
    )
    .expect("Failed to styled.");

    let value = use_state(|| String::from(""));
    let on_input = {
        let value = value.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            value.set(input.value());
        })
    };

    let html = cmark(value.to_string());
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(&html);

    let node = Node::from(div);
    let vnode = VNode::VRef(node);

    html! {
        <>
        <h1>{"Markdown Editor"}</h1>
        <div class="markdown-body">
            <div class={classes!(container)}>
                <div class={classes!(split.clone(), split_input)}>
                    <textarea class={classes!(text)} rows="100%" cols="100%" value={value.to_string()} oninput={on_input} />
                </div>
                <div class={classes!(split.clone(), split_output)} >
                    {vnode}
                </div>
            </div>
        </div>
        </>
    }
}

fn cmark(text: String) -> String {
    let mut options = Options::empty();
    options.insert(
        Options::ENABLE_TABLES
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_SMART_PUNCTUATION,
    );
    let parser = Parser::new_ext(&text, options);

    let mut html_output = String::new();
    // parser changes rendered String for markdown
    html::push_html(&mut html_output, parser);

    html_output
}
