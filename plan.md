# 画像ビューアー 実装計画

`requirement.md`（要件定義）・`spec.md`（設計）に基づく実装タスクの計画。フェーズ順に上から実施する。

## フェーズ0: 開発環境準備
- [ ] Rust toolchain インストール確認（`rustc --version`）
- [ ] Node.js / npm インストール確認
- [ ] Tauri CLI 前提条件確認（Windows: WebView2 Runtime, MSVC Build Tools）

## フェーズ1: プロジェクト初期化
- [ ] `npm create tauri-app@latest` で Svelte + TypeScript テンプレートを使いプロジェクト作成（プロジェクト名: `imv`）
- [ ] 依存パッケージ追加
  - フロントエンド: `@tauri-apps/plugin-dialog`
  - バックエンド (Cargo): `walkdir`（再帰的ディレクトリ走査）, `dirs`（ホームディレクトリ取得）
- [ ] `spec.md` の「2. プロジェクト構成」に合わせてディレクトリ・ファイルの雛形を作成（`src-tauri/src/scan.rs`, `src-tauri/src/trash.rs`, `src/lib/components/`, `src/lib/stores/`）
- [ ] `npm run tauri dev` で雛形アプリが起動することを確認

## フェーズ2: バックエンド実装（Rust）
- [ ] `scan.rs`: `scan_images(path: String) -> Result<Vec<String>, String>` を実装
  - [ ] ファイル/ディレクトリ判定 → 基準ディレクトリ決定
  - [ ] `walkdir` で再帰走査、隠しファイル/隠しディレクトリを除外
  - [ ] 対応拡張子（jpg/jpeg/png/gif/bmp/webp、大文字小文字無視）でフィルタ
  - [ ] 結果0件でもエラーにせず空配列を返す
- [ ] `trash.rs`: `move_to_trash(path: String) -> Result<String, String>` を実装
  - [ ] `dirs::home_dir()` 直下の `Trash` フォルダを取得、なければ作成
  - [ ] 同名ファイル衝突時に `_1`, `_2`... の連番サフィックスを付与するロジック
  - [ ] `std::fs::rename` でファイル移動、失敗時は `Err(String)` を返す
- [ ] `main.rs`: 上記2コマンドを `tauri::generate_handler!` に登録
- [ ] `tauri.conf.json` / `capabilities` 設定: asset protocol が任意の選択ディレクトリを読めるようスコープを設定（`spec.md` 6章）
- [ ] `dialog` プラグインを Tauri 側に登録（`tauri-plugin-dialog`）

## フェーズ3: フロントエンド基盤
- [ ] `imageStore.ts` 実装（`files: string[]`, `current: string | null` を保持する Svelte store）
  - [ ] `pickRandom()`
  - [ ] `next()`
  - [ ] `deleteCurrent()`（`move_to_trash` invoke → 一覧から除去 → 再選択 or 空状態）
- [ ] `App.svelte`: 画面状態（起動時選択 / ビューア / 空状態）の切り替えロジック

## フェーズ4: 画面・コンポーネント実装
- [ ] 起動時: `@tauri-apps/plugin-dialog` の `open()` でファイル/フォルダ選択 → `scan_images` invoke
- [ ] `ImageViewer.svelte`: `convertFileSrc` で画像URL生成、`object-fit: contain` で表示
- [ ] `Controls.svelte`: 「次へ」「削除」ボタン、残枚数表示
- [ ] `EmptyState.svelte`: 0件/全削除時のメッセージ＋「フォルダを選び直す」ボタン
- [ ] キーボードショートカット実装（→ / Space = 次へ、Delete = 削除）

## フェーズ5: フロー結線・エラーハンドリング
- [ ] 起動〜初回表示フロー結線（`spec.md` 5.1）
- [ ] 「次へ」操作フロー結線（`spec.md` 5.2）
- [ ] 「削除」操作フロー結線（`spec.md` 5.3）
- [ ] `move_to_trash` / `scan_images` 失敗時のエラー表示（トースト等、画面遷移はしない）

## フェーズ6: ウィンドウ・見た目調整
- [ ] 初期ウィンドウサイズ 1024×768、リサイズ可に設定（`tauri.conf.json`）
- [ ] 背景色・レイアウト調整（画像を中央に、アスペクト比維持）

## フェーズ7: 動作確認（手動テスト）
- [ ] ディレクトリ指定で起動 → サブディレクトリ含め画像が検出されること
- [ ] ファイル指定で起動 → そのファイルの親ディレクトリ配下が対象になること
- [ ] 対応形式（jpg/jpeg/png/gif/bmp/webp）がすべて表示できること
- [ ] GIFがアニメーション再生されること
- [ ] 「次へ」で毎回ランダムに選ばれること（同じ画像の連続表示もあり得ることを確認）
- [ ] 「削除」で `~/Trash` にファイルが移動すること
- [ ] トラッシュ内に同名ファイルがある場合に連番サフィックスが付くこと
- [ ] 対象ファイル0件のときに空状態画面が出ること
- [ ] 全件削除後に空状態画面へ遷移すること
- [ ] キーボードショートカットが機能すること
- [ ] 隠しファイル/隠しディレクトリが除外されること

## フェーズ8: パッケージング
- [ ] `npm run tauri build` で Windows インストーラ/実行ファイルを生成
- [ ] 生成物を実環境（開発環境以外のWindows PC等）で動作確認

## 備考
- 各フェーズはおおむね上から順に依存するが、フェーズ2（バックエンド）とフェーズ3〜4（フロントエンド基盤・コンポーネント）は並行して着手可能。
- `spec.md` 7章の「未確定事項への対応まとめ」は暫定決定のため、実装中に不都合があれば都度 `requirement.md` / `spec.md` を更新してから実装を修正する。
