use pulldown_cmark::{html, Options, Parser};
use web_sys::HtmlTextAreaElement;
use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;

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
        <div class="container">
            <div class="item">
                <textarea rows="140" cols="100" value={value.to_string()} oninput={on_input} />
            </div>
            <div class="item" >
                {vnode}
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
