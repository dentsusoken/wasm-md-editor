# wasm-md-editor
wasm-md-editorはReactベースでMarkdownのparserに[pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)を利用したMarkdownエディタです。

# WebAssembly
ブラウザで稼働するバイナリ形式のアセンブリ言語。

# Wasm Bundler 
## wasm-pack
WebAssemblyのコンパイルや、パッケージのビルドにはwasm-packを利用。
wasm-packは内部で別のツールであるwasm-bindgenを利用して、JavascriptとRust間の疎通を実現している。
#[wasm_bindgen]アトリビュートにより、Rustの実装をJavascriptが理解できるような形で``pkg``配下にjsファイルが作成される

````rust
#[wasm_bindgen]
pub fn wasm_main(text: &str) -> String {
    let mut options = Options::empty();
    options.insert(
        Options::ENABLE_TABLES
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_SMART_PUNCTUATION,
    );
    let parser = Parser::new_ext(text, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

````

````
pkg/
├── package.json
├── wasm_md_editor.js
├── wasm_md_editor_bg.js
├── wasm_md_editor_bg.wasm
└── wasm_md_editor_bg.wasm.d.ts
````

- wasm_md_editor_bg.wasm
  - Rustコンパイラによって生成されるWebAssemblyバイナリ。
- wasm_md_editor.js
  - [#wasm-bindgen]によって生成されるjsファイル。Wasmの関数をJavascriptで利用可能にするためのブリッジ的な役割。
- wasm_md_editor_bg.wasm.d.ts
  - 型宣言を含んだブリッジ。Typescriptを使っている場合には、Wasm関数を利用する際に型チェックやIDEによる補完が可能


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


