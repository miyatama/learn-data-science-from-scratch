# タスクリスト - Version 1.0.2

## タスク一覧

### 実装

- [ ] `Cargo.toml` の version を `1.0.2` に更新
- [ ] `src/statistics.rs` に `min` / `max` 関数を追加
  - [ ] `min` 関数
  - [ ] `max` 関数
  - [ ] `data_range` を `min` / `max` を使うよう整理
  - [ ] `min` / `max` の単体テストを追加
- [ ] `src/main.rs` の散らばりセクションを拡張
  - [ ] 最小値を出力
  - [ ] 最大値を出力
  - [ ] 数値の範囲を出力
- [ ] `src/main.rs` に 3σフィルタ後の相関を追加
  - [ ] 3σフィルタロジックを実装
  - [ ] 3σ範囲内の共分散を出力
  - [ ] 3σ範囲内の相関係数を出力

### 品質チェック

- [ ] `cargo build --release` が成功すること
- [ ] `cargo test` が全パスすること
- [ ] `cargo clippy` で警告ゼロ
- [ ] `./target/release/statistics ./examples/statistics.json` で正しく出力されること
- [ ] `-h / --help` が従来通り動作すること

## 完了条件

- 全タスクのチェックが埋まっていること
- 受け入れ基準（requirements.md）をすべて満たしていること
