# wasm-md-editor
wasm-md-editorはフロントエンドに[Yew](https://yew.rs/ja/)を、Markdownのparserに[pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)を利用したMarkdownエディタです。

## Yewとは
- WebAssemblyによるRust製フロントエンドフレームワーク
- ReactのJSXライクなHTMLマクロ
- 仮想DOMを使ったレンダリング、Reactに寄せた状態管理機構

# WebAssembly
ブラウザで稼働するバイナリ形式のアセンブリ言語。軽量で高速、特定の環境や特定のプログラミング言語をターゲットとしないので、様々な場面で利用できる。（特にRustは）




## Trunk
- wasm-packはRustのソースコードをJsコードにコンパイルし、Jsモジュールで利用可能にするためのバンドルツール
- TrunkはRustのコードをJSやその他のアセット(html,image, css)にまとめるためのバンドルツール

## Yew 

### Callback
Callbackを利用することで、画面のイベント処理時にコンポーネントやDOMと非同期的に通信できる。


````rs
#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model { value: 0 });
    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1,
            })
        })
    };
````

Callbackは
````rs
pub enum Callback<IN> {
    /// A callback which can be called multiple times with optional modifier flags
    Callback {
        /// A callback which can be called multiple times
        cb: Rc<dyn Fn(IN)>,

        /// Setting `passive` to [Some] explicitly makes the event listener passive or not.
        /// Yew sets sane defaults depending on the type of the listener.
        /// See
        /// [addEventListener](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener).
        passive: Option<bool>,
    },

    /// A callback which can only be called once. The callback will panic if it is
    /// called more than once.
    CallbackOnce(Rc<CallbackOnce<IN>>),
}

impl<IN, F: Fn(IN) + 'static> From<F> for Callback<IN> {
    fn from(func: F) -> Self {
        Callback::Callback {
            cb: Rc::new(func),
            passive: None,
        }
    }
}

````


# TODO
- Previewつくる
- https://docs.rs/pulldown-cmark/latest/pulldown_cmark/  をもとにExampleソース書いてMarkdownのレンダリングを試してみる