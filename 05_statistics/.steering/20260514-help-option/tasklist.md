# タスクリスト - Version 1.0.1 ヘルプ機能実装

## タスク一覧

### 実装

- [ ] `Cargo.toml` の version を `1.0.1` に更新
- [ ] `src/main.rs` に `print_help()` 関数を追加
- [ ] `src/main.rs` の引数解析に `-h` / `--help` 判定を追加

### 品質チェック

- [ ] `cargo build --release` が成功すること
- [ ] `cargo test` が全パスすること
- [ ] `cargo clippy` で警告ゼロ
- [ ] `./target/release/statistics -h` でヘルプが表示されること
- [ ] `./target/release/statistics --help` でヘルプが表示されること
- [ ] `./target/release/statistics ./examples/statistics.json` で従来通り動作すること

## 完了条件

- 全タスクのチェックが埋まっていること
- 受け入れ基準（requirements.md）をすべて満たしていること
