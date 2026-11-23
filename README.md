# imv（画像ビューアー）

指定したフォルダまたはファイル配下の画像を、サブディレクトリも含めて検索し、ランダムに1枚ずつ表示するWindowsデスクトップアプリです。不要な画像はその場で削除（トラッシュフォルダへ移動）でき、写真の選別作業を効率化します。

Tauri（Rust）+ SvelteKit（TypeScript）で実装しています。

## 主な機能

- フォルダまたは画像ファイルを指定して、配下の画像をランダム表示
- 「次へ」「前へ」で表示履歴を進んだり戻ったりできる
- 「削除」で画像をトラッシュフォルダへ移動（初期設定は `~/tmp/Trash`、設定画面から変更可能）
- 「取り消し」ボタン（`u` キー）で直近に削除した画像を元の場所へ戻す（連続して押すと、さかのぼって複数回取り消せる）
- 最近開いたフォルダ/ファイルの履歴から再度開ける
- キーボードショートカット: `←`・`h` 前へ / `→`・`Space`・`l` 次へ / `Delete`・`d` 削除 / `u` 削除の取り消し / `q` 終了
- 対応形式: `jpg` / `jpeg` / `png` / `gif`（アニメーション再生）/ `bmp` / `webp`

## 動作環境・前提条件

- Windows
- [Node.js](https://nodejs.org/)（v18以上推奨）
- [Rust](https://www.rust-lang.org/tools/install)（`rustc` / `cargo`）
- Tauriのビルドに必要なWindows側の依存関係（[Tauri公式の前提条件](https://tauri.app/start/prerequisites/)を参照。WebView2 RuntimeとMSVC Build Toolsが必要です）

## セットアップ

```sh
npm install
```

## 開発時の実行

開発用サーバーとアプリウィンドウを起動します（ソース変更はホットリロードされます）。

```sh
npm run tauri dev
```

## ビルド

Windows向けの実行ファイル・インストーラを生成します。

```sh
npm run tauri build
```

成功すると以下に生成されます。

- 単体実行ファイル: `src-tauri/target/release/imv.exe`
- MSIインストーラ: `src-tauri/target/release/bundle/msi/imv_<version>_x64_en-US.msi`
- NSISインストーラ: `src-tauri/target/release/bundle/nsis/imv_<version>_x64-setup.exe`

## その他のコマンド

```sh
npm run check   # 型チェック（svelte-check）
cargo test      # Rustバックエンドのユニットテスト（src-tauri ディレクトリ内で実行）
```

## アプリアイコン

元画像は `app-icon.png`（1024×1024）で、`tools/make_icon.py` で描画しています。変更する場合は以下で再生成します。

```sh
python tools/make_icon.py app-icon.png   # 元画像を生成（Pillow が必要）
npm run tauri icon app-icon.png          # src-tauri/icons 配下の各サイズを生成
```

`tauri icon` は Android / iOS 用のアイコンも生成しますが、本アプリでは使わないため `src-tauri/icons/android`・`src-tauri/icons/ios` は削除してください。

## ドキュメント

- [requirement.md](./requirement.md): 要件定義
- [spec.md](./spec.md): 設計書
- [plan.md](./plan.md): 実装計画

## 推奨IDE環境

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
