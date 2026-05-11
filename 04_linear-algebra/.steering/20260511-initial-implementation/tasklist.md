# タスクリスト — Version 1.0.0 初回実装

## タスク一覧

### 1. プロジェクト初期化
- [ ] `Cargo.toml` を作成する

### 2. ベクトル演算モジュール実装（`src/vector.rs`）
- [ ] `check_same_length` ヘルパー関数を実装する
- [ ] `vector_add` を実装する
- [ ] `vector_subtract` を実装する
- [ ] `scalar_multiply` を実装する
- [ ] `dot_product` を実装する
- [ ] `sum_of_squares` を実装する
- [ ] `magnitude` を実装する

### 3. ユニットテスト実装（`src/vector.rs` 内）
- [ ] `test_vector_add_normal` を実装する
- [ ] `test_vector_add_dimension_mismatch` を実装する
- [ ] `test_vector_subtract_normal` を実装する
- [ ] `test_vector_subtract_dimension_mismatch` を実装する
- [ ] `test_scalar_multiply` を実装する
- [ ] `test_dot_product_normal` を実装する
- [ ] `test_dot_product_dimension_mismatch` を実装する
- [ ] `test_sum_of_squares` を実装する
- [ ] `test_magnitude` を実装する

### 4. CLIエントリーポイント実装（`src/main.rs`）
- [ ] 引数パース処理を実装する
- [ ] `add` サブコマンドを実装する
- [ ] `sub` サブコマンドを実装する
- [ ] `scale` サブコマンドを実装する
- [ ] `dot` サブコマンドを実装する
- [ ] `sumsq` サブコマンドを実装する
- [ ] `magnitude` サブコマンドを実装する

### 5. 品質チェック
- [ ] `cargo test` が全て通過することを確認する
- [ ] `cargo clippy` でワーニングがないことを確認する
- [ ] `cargo fmt` でフォーマットを統一する
- [ ] 受け入れ基準を手動で確認する

## 完了条件

- `cargo test` が全テスト通過
- 受け入れ基準の全コマンドが正しい結果を返す
- `cargo clippy` でワーニングなし
