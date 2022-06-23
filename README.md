# wasm-md-editor
wasm-md-editorはフロントエンドに[Yew](https://yew.rs/ja/)を、Markdownのparserに[pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)を利用したMarkdownエディタ。
![image](https://user-images.githubusercontent.com/57422625/174615121-9c00b2e1-23e2-4877-9486-5eb8a4726cf7.png)

#### Table of contents
- [wasm-md-editor](#wasm-md-editor)
      - [Table of contents](#table-of-contents)
  - [Yew](#yew)
    - [Yew内部で使われる主要ライブラリ](#yew内部で使われる主要ライブラリ)
    - [wasm-bindgen](#wasm-bindgen)
- [WebAssembly](#webassembly)
  - [Trunk](#trunk)


## Yew
主な特徴は下記。
- WebAssemblyによるRust製フロントエンドフレームワーク
- ReactのJSXライクなHTMLマクロ
- 仮想DOMを使ったレンダリング、Reactに寄せた状態管理機構

### Yew内部で使われる主要ライブラリ
- [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/)
  - Rustで書いたコード(関数)をJavascript側で利用するためのもの。以下のjs-sysとweb-sysを使ったRustコードもwasm-bindgenが最終的にjavascript側にexportしている。
- [js-sys](https://docs.rs/js-sys/0.3.58/js_sys/)
  - [Javascriptの標準ビルトインオブジェクト](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects)をRustに提供している。
- [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)
  - ブラウザが提供するWeb APIをRustに提供している。
- [wasm-bindgen-futures](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/)
  - JavascriptのPromiseをRustのFuture型として操作することができるブリッジの役割を果たす。ビルド時にJavascriptのPromiseに変換する。

### wasm-bindgen
#[wasm-bindgen]アトリビュートはRustとJavascriptを変換することができ、Javascriptの型をRustのコードで表現することができる。(Javascript→Rustも可)  
wasm-bindgen内で使われるweb-sys, js-sysがRustのコードをWebAPIやJavascriptAPIにバインドすることでこれを実現している。




TODO:
- bindgenマクロ当たりを追記する。Yew内部で使っている部分。https://yew.rs/ja/docs/concepts/wasm-bindgen
  - https://github.com/yewstack/yew/blob/master/packages/yew-macro/src/lib.rs
- WebAssemblyの説明追記と、バンドラの比較記載
- ソースコード解説
- YewのStyleについて追記




# WebAssembly
ブラウザで稼働するバイナリ形式のアセンブリ言語。軽量で高速、特定の環境や特定のプログラミング言語をターゲットとしないので、様々な場面で利用できる。



## Trunk

- wasm-packはRustのソースコードをJsコードにコンパイルし、Jsモジュールで利用可能にするためのバンドルツール
- TrunkはRustのコードをJSやその他のアセット(html,image, css)にまとめるためのバンドルツール
