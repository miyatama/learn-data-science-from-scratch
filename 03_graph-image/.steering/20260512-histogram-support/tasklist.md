# tasklist.md

## タスクリスト

- [ ] 1. `src/parser.rs` に `HistogramData` 構造体と `parse_histogram` 関数を追加
- [ ] 2. `src/renderer/histogram.rs` を新規作成（ビン集計 + plotters描画）
- [ ] 3. `src/renderer/mod.rs` に `pub mod histogram;` を追加
- [ ] 4. `src/main.rs` に `histogram` アームを追加
- [ ] 5. `cargo build` でビルド確認
- [ ] 6. `cargo test` でテスト確認
- [ ] 7. `examples/histgram.json` を使った動作確認

## 完了条件

- `cargo build` が通ること
- `cargo test` が通ること
- `--type histogram -i examples/histgram.json` でヒストグラム画像が出力されること
