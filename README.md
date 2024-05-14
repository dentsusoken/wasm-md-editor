# wasm-md-editor
wasm-md-editorはフロントエンドに[Yew](https://yew.rs/ja/)を、Markdownのparserに[pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)を利用したMarkdownエディタ。
![image](https://user-images.githubusercontent.com/57422625/174615121-9c00b2e1-23e2-4877-9486-5eb8a4726cf7.png)

# Run
[Trunk](https://github.com/thedodd/trunk)を利用。
````bash
$ trunk serve
````


# Tips
https://github.com/trunk-rs/trunk/issues/443 のようにtrunk serveでwasm-bindgenのダウンロードに失敗する場合、手元のwasm-bindgenコマンドとソース上のバイナリのバージョンが一致していない可能性あり。
（手元のwasm-bindgenをtarを落としてきてC/users直下に置いたら治った）