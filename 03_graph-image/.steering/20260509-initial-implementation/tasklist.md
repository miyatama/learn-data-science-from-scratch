# 初回実装 タスクリスト

## ステータス凡例
- [ ] 未着手
- [x] 完了

---

## 1. プロジェクトセットアップ

- [x] 1-1. `cargo init` でRustプロジェクトを初期化する
- [x] 1-2. `Cargo.toml` に依存クレートを追加する（clap, serde, serde_json, plotters, anyhow）
- [x] 1-3. `examples/` ディレクトリを作成する
- [x] 1-4. `.gitignore` に `output/` が含まれていることを確認・追加する

## 2. サンプルJSONファイル作成

- [x] 2-1. `examples/line.json` を作成する
- [x] 2-2. `examples/bar.json` を作成する
- [x] 2-3. `examples/scatter.json` を作成する

## 3. データ構造・パーサー実装

- [x] 3-1. `src/parser.rs` を作成する（DataPoint, Series, LineData, BarData, ScatterData）
- [x] 3-2. `src/parser.rs` の単体テストを追加する

## 4. InputReader実装

- [x] 4-1. `src/input.rs` を作成する（ファイル読み込み・標準入力読み込み）
- [x] 4-2. `src/input.rs` の単体テストを追加する

## 5. Rendererトレイト・共通型実装

- [x] 5-1. `src/renderer/mod.rs` を作成する（Rendererトレイト定義）

## 6. 折れ線グラフRenderer実装

- [x] 6-1. `src/renderer/line.rs` を作成する
- [x] 6-2. `src/renderer/line.rs` の単体テストを追加する

## 7. 棒グラフRenderer実装

- [x] 7-1. `src/renderer/bar.rs` を作成する
- [x] 7-2. `src/renderer/bar.rs` の単体テストを追加する

## 8. 散布図Renderer実装

- [x] 8-1. `src/renderer/scatter.rs` を作成する
- [x] 8-2. `src/renderer/scatter.rs` の単体テストを追加する

## 9. CLIエントリポイント実装

- [x] 9-1. `src/main.rs` にCLI定義を実装する（clap derive API）
- [x] 9-2. `src/main.rs` にグラフタイプ振り分けロジックを実装する

## 10. 結合テスト

- [x] 10-1. `tests/integration_test.rs` を作成する
- [x] 10-2. 各グラフタイプの結合テストを実装する

## 11. 品質チェック

- [x] 11-1. `cargo fmt` を実行する
- [x] 11-2. `cargo clippy -- -D warnings` を実行し警告ゼロを確認する
- [x] 11-3. `cargo test` を実行しすべてパスを確認する
- [x] 11-4. `cargo build --release` でビルド成功を確認する

## 12. 動作確認

- [x] 12-1. `examples/line.json` でPNG生成を確認する
- [x] 12-2. `examples/bar.json` でPNG生成を確認する
- [x] 12-3. `examples/scatter.json` でPNG生成を確認する
- [x] 12-4. 標準入力からJSONを渡してPNG生成を確認する
- [x] 12-5. 存在しない出力ディレクトリへの自動作成を確認する

## 完了条件

`requirements.md` の受け入れ基準がすべて満たされていること。
