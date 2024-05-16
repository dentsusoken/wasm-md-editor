# Tauriとは
Rust製の軽量なデスクトップアプリ開発フレームワーク。
従来はElectronが主流（vscode, discordが代表例）だったが、メモリ・CPU使用量、アプリのバンドルサイズがより軽量化されたTauriが注目を集めている。

- ElectronはChromiumを使ってレンダリングを行うため思い
- TauriはWebViewライブラリのWRYを使うことでOS標準のWebView機能を利用

Gitのスター数もかなりElectronをまくる勢い
![](./img/star-history-2024516.png)

# Tauriのアーキテクチャ
![a](./img/tauri-archi.png)
https://tauri.app/v1/references/architecture/


## WRY

# 実施手順


````bash
$ cargo install tauri-cli
$ cargo tauri init

warning: `C:\Users\li3248\.cargo\config` is deprecated in favor of `config.toml`
note: if you need to support cargo 1.38 or earlier, you can symlink `config` to `config.toml`
warning: `C:\Users\li3248\.cargo\config` is deprecated in favor of `config.toml`
note: if you need to support cargo 1.38 or earlier, you can symlink `config` to `config.toml`
✔ What is your app name? · wasm-md-editor
✔ What should the window title be? · md-editor
✔ Where are your web assets (HTML/CSS/JS) located, relative to the "<current dir>/src-tauri/tauri.conf.json" file that will be created? · ../dist 
✔ What is the url of your dev server? · http://127.0.0.1:8080
✔ What is your frontend dev command? · trunk serve
✔ What is your frontend build command? · trunk build
````

フォルダ構成図
````
wasm-md-editor
├─dist
├─doc
│  └─img
├─public
├─src
│  ├─components
│  └─pages
├─src-tauri ※新規作成される
│  ├─icons
│  └─src
└─target
````

# Tauri development window

````bash
$ cargo tauri dev
````

![](./img/home.png)
