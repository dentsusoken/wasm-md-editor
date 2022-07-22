# wasm-md-editor
wasm-md-editorはフロントエンドに[Yew](https://yew.rs/ja/)を、Markdownのparserに[pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)を利用したMarkdownエディタ。
![image](https://user-images.githubusercontent.com/57422625/174615121-9c00b2e1-23e2-4877-9486-5eb8a4726cf7.png)

#### Table of contents
- [wasm-md-editor](#wasm-md-editor)
      - [Table of contents](#table-of-contents)
- [WebAssembly](#webassembly)
  - [Trunk](#trunk)






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
