# タスクリスト — Version 2.0.0 行列操作

## タスク一覧

### 1. Cargo.toml 更新
- [ ] バージョンを `2.0.0` に更新する

### 2. 行列演算モジュール実装（`src/matrix.rs`）
- [ ] `shape` 関数を実装する

### 3. ユニットテスト実装（`src/matrix.rs` 内）
- [ ] `test_shape_normal` を実装する（2行3列）
- [ ] `test_shape_square` を実装する（正方行列）
- [ ] `test_shape_empty` を実装する（空行列）
- [ ] `test_shape_column_mismatch` を実装する（列数不一致エラー）

### 4. `src/main.rs` 更新
- [ ] `parse_matrix` 関数を実装する
- [ ] `matrix` サブコマンドのルーティングを追加する
- [ ] `matrix shape` サブコマンドを実装する
- [ ] `matrix --help` を実装する
- [ ] `matrix shape --help` を実装する
- [ ] `mod matrix;` を追加する

### 5. 永続的ドキュメント更新
- [ ] `docs/functional-design.md` に行列モジュールの情報を追記する

### 6. 品質チェック
- [ ] `cargo test` が全て通過することを確認する
- [ ] `cargo clippy` でワーニングがないことを確認する
- [ ] `cargo fmt` でフォーマットを統一する
- [ ] 受け入れ基準を手動で確認する

## 完了条件

- `cargo test` が全テスト通過
- `matrix shape` コマンドが正しい shape を返す
- 列数不一致時にエラーを表示する
- ヘルプが正しく表示される
- `cargo clippy` でワーニングなし
