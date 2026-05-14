# タスクリスト - Version 1.0.0 初回実装

## タスク一覧

### セットアップ

- [ ] Cargo.toml を作成（パッケージ名 `statistics`、serde/serde_json 依存）

### 実装

- [ ] `src/statistics.rs` を作成
  - [ ] `mean`
  - [ ] `median`
  - [ ] `quantile`
  - [ ] `mode`
  - [ ] `data_range`
  - [ ] `variance`
  - [ ] `std_deviation`
  - [ ] 各関数の単体テスト
- [ ] `src/correlation.rs` を作成
  - [ ] `covariance`
  - [ ] `correlation`
  - [ ] 各関数の単体テスト
- [ ] `src/main.rs` を作成
  - [ ] 引数解析（ファイルパス）
  - [ ] JSON 読み込み・パース
  - [ ] 代表値セクション出力
  - [ ] 散らばりセクション出力
  - [ ] 相関セクション出力（datas >= 2 の場合）
  - [ ] エラーハンドリング

### 品質チェック

- [ ] `cargo build --release` が成功すること
- [ ] `cargo test` が全パスすること
- [ ] `cargo clippy` で警告ゼロ
- [ ] `./target/release/statistics ./examples/statistics.json` で正しく出力されること

## 完了条件

- 全タスクのチェックが埋まっていること
- 受け入れ基準（requirements.md）をすべて満たしていること
