<h1>TOC</h1>

- [本資料の目的](#本資料の目的)
- [背景](#背景)
  - [採用した主要ライブラリ](#採用した主要ライブラリ)
  - [WebAssemblyとは？](#webassemblyとは)
  - [Yew内部で使われる主要ライブラリ](#yew内部で使われる主要ライブラリ)
    - [wasm-bindgen](#wasm-bindgen)
  - [YewでどのようにComponentを定義しているのか](#yewでどのようにcomponentを定義しているのか)
  - [``#[function_component]``と``html!``](#function_componentとhtml)
    - [``#[function_component]``](#function_component)
    - [``html!``](#html)
  - [まとめ](#まとめ)

# 本資料の目的
- Yewの仕組みをざっくり理解する
- YewをもとにRustのマクロ(主に手続き型マクロ)の理解をする


# 背景
WebAssemblyを触ってみたい。Rustでフロントエンドの実装ができるらしい。  
⇒シンプルなUIでMarkdownエディタを作ってみよう。

  
## 採用した主要ライブラリ
- Yew
  - WebAssemblyによるRust製フロントエンドフレームワーク
  - ReactのJSXライクなHTMLマクロや状態管理機構を持つ
- pulldown-cmark
  - Markdown記法のテキストをHTML形式に変換するparser


##  WebAssemblyとは？
ブラウザ上で動作するバイナリ―形式のアセンブリ言語。ネイティブアプリに近いパフォーマンスで動作することができる言語と言われている。
Javascriptを補完、並行して動作するように設計されており、WebAssemblyモジュールをJavascriptアプリケーションで読み込み、それぞれの間で機能(関数)を共有することができる。
現在WebAssemblyにコンパイルできる言語には、C/C++、Rustがある。



## Yew内部で使われる主要ライブラリ
- [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/)
  - Rustで書いたコード(関数)をJavascript側で利用するためのもの。以下のjs-sysとweb-sysを使ったRustコードもwasm-bindgenが最終的にjavascript側にexportしている。実際にYew内部ではイベントリスナーや、仮想DOMの更新等の動的な処理でwasmを利用している。
- [js-sys](https://docs.rs/js-sys/0.3.58/js_sys/)
  - [Javascriptの標準ビルトインオブジェクト](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects)をRustに提供している。
- [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)
  - ブラウザが提供するWeb APIをRustに提供している。
- [wasm-bindgen-futures](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/)
  - JavascriptのPromiseをRustのFuture型として操作することができるブリッジの役割を果たす。ビルド時にJavascriptのPromiseに変換する。


### wasm-bindgen
#[wasm-bindgen]アトリビュートはRustとJavascriptを相互に変換することができ、Javascriptの型をRustのコードで表現することが可能。
wasm-bindgen内で使われるweb-sys, js-sysがRustのコードをWebAPIやJavascriptAPIにバインドすることでこれを実現している。



## YewでどのようにComponentを定義しているのか
Yewでは``#[function_component]``を付与した関数を定義することによってコンポーネントの実装を行う。

````rs
use yew::prelude::*;

// これはHomeコンポーネントとして認識される
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <h1>{"Welcome to my editor!"}</h1>
    }
}
````
この時に利用した``#[function_component]``をアトリビュートと呼ぶ。
``html!``マクロでは、インナーブロックで与えられたHTMLタグを処理し、HTMLとして返却する。


## ``#[function_component]``と``html!``

### ``#[function_component]``
Yewでは、コンポ－ネントの定義には``#[function_component]``、JSX記法でHTML要素を定義する際には``html!``を使用した。
どちらも記法は異なるが、Rustではどちらもマクロと呼ばれる。
ただし、``#[function_component]``のように関数や構造体に付与するものは手続き的マクロ、``println!``のようにmacro_rules!で定義され、呼び出し元からは関数呼び出しのように呼ばれるものは宣言的マクロと分類される。
``html!``は呼び出し元から関数のように呼ばれる点では``println!``と同じように見えるが、定義の仕方が明確に違っており、こちらは手続き型マクロ（の関数風マクロ）に分類される。（詳細は後述）



#[function_component]アトリビュートの中身はこれ。
````rs
#[proc_macro_attribute]
pub fn function_component(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as FunctionComponent);
    let attr = parse_macro_input!(attr as FunctionComponentName);

    function_component_impl(attr, item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
````
- ``proc_macro_attribute``がfunction_component()関数がCustom Attributeであることを示しているため、利用側で#[function_component]とアトリビュートを付与した際にこの関数がリンクされる。
- 手続き的マクロを定義する関数には、``TokenStream``を入力として受け取り``TokenStream``を出力として返す。
- アトリビュートを付与したソースコードが入力値としてTokenStreamに変換され、それを基にマクロが生成するソースコードがTokenStreamとして返却される。 
- 引数の１つ目である``attr: proc_macro::TokenStream``は呼び出し側(``#[function_component(Home)]``)の``Home``を指しているのに対し、2つ目の``item: proc_macro::TokenStream``は``#[function_component(Home)]``を付与した関数の中身に対応している。(function_componentの例ではitemはFunctionComponent、attrはFunctionComponentNameに対応)
- ``parse_macro_input!``はTokenStreamのトークン列を構文木にパース。Rustが解釈できるデータ構造に変換。
- その後データ構造が更新され、TokenStreamに再変換して呼び出し元に返却。

一般的に、parse_macro_input!によって構文木にパースされたTokenStreamは、このあと``quote``マクロによって再度TokenStreamに変換され、マクロ呼び出し元の結果として返却される。
マクロを定義するlib.rsにはアトリビュートに関する詳細な実装を書かず、アトリビュート本体の実装は別のクレートの関数を呼び出す形式をとることが多い。
Yewの#[functionComponent]においてもその流れは変わらず、上述の流れでパースされた構文木は``function_componnet_impl``関数内でquote!マクロが呼ばれてトークン列に変換されている。

````rs
pub fn function_component_impl(
    name: FunctionComponentName,
    component: FunctionComponent,
) -> syn::Result<TokenStream> {
    let FunctionComponentName { component_name } = name;

    // ・・・略

    let quoted = quote! {
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #[allow(unused_parens)]
        #vis struct #function_name #impl_generics {
            _marker: ::std::marker::PhantomData<(#phantom_generics)>,
        }

        impl #impl_generics ::yew::functional::FunctionProvider for #function_name #ty_generics #where_clause {
            type TProps = #props_type;

            fn run(#arg) -> #ret_type {
                #block
            }
        }

        #(#attrs)*
        #[allow(type_alias_bounds)]
        #vis type #component_name #impl_generics = ::yew::functional::FunctionComponent<#function_name #ty_generics>;
    };

    Ok(quoted)
}

````

### ``html!``
html!のようなマクロは手続き的マクロの一種で、関数風マクロとも呼ばれる。宣言的マクロと非常に似ているが、マクロの定義では``macro_rules!``ではなく、``#[proc_macro]``を利用する点で違いがある。（手続き型マクロがRustのバージョン1.15.0で追加されたため、宣言マクロと関数風マクロは似ているが、関数風マクロが後発）


html!を使ったサンプル実装を下記に示す。

````rs
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
            <h1>{"Welcome to my editor!"}</h1>
    }
}

````

````rs
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

````
- Homeコンポーネントは「Welcome to my editor!」を返すだけのシンプルなコンポーネント。
- Topコンポーネントは、Homeコンポーネントを呼び出しており、Startボタンを配置するTopページを表している。
- styleは[stylist](https://github.com/futursolo/stylist-rs)を利用しており、Reactのような宣言的なスタイルの定義が可能。

ここまでの説明でやっと冒頭の画像のコンポーネントの画像に戻る。
![welcome](/welcome-my-editor.png)

````rs
#[proc_macro_error::proc_macro_error]
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as HtmlRootVNode);
    TokenStream::from(root.into_token_stream())
}
````

- html!マクロも同様、TokenStreamを受け取って構文木に変換
- その後再度TokenStreamに変換しマクロ呼び出し元に返却する

## まとめ
- 手続き的マクロでは、ソースコードをコピーしてマクロに渡し、TokenStreamから構文木に変換。それを再度TokenStreamに型を戻したのちにマクロ呼び出し元に返却する尾という流れでソースコードを出力していた。これによって開発者はコードの記述量を減らし、楽をすることが可能になった。
- Rustのマクロは、呼び出し側での使い勝手はかなり便利になる反面、どういう状況下でマクロを作ってもよいかを適切に判断しないと、マクロ量産されてメンテナンスに苦しむことになりそう。呼び出し側でコーディングする量が激減する、宣言的にアトリビュートとして付与できる、macro_rules!による宣言的マクロで、関数として定義するよりも可読性も向上する、等のメリットを享受できるのであればマクロを利用することはいいかもしれない。
