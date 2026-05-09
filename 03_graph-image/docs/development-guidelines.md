# 開発ガイドライン

## コーディング規約

- Rust edition 2021 を使用する
- `rustfmt` のデフォルト設定に従いフォーマットする（`cargo fmt`）
- `clippy` の警告をゼロに保つ（`cargo clippy -- -D warnings`）
- `unsafe` コードは使用しない
- `unwrap()` / `expect()` はテストコード以外では使用しない。`?` 演算子または `anyhow` で伝播させる

## 命名規則

| 種別 | 規則 | 例 |
|---|---|---|
| モジュール名 | snake_case | `line_renderer` |
| 構造体・トレイト・Enum | UpperCamelCase | `LineRenderer`, `GraphData` |
| 関数・メソッド・変数 | snake_case | `render_graph`, `output_path` |
| 定数 | SCREAMING_SNAKE_CASE | `DEFAULT_SIZE` |
| ファイル名 | snake_case | `line.rs`, `bar.rs` |

## エラーハンドリング規約

- エラー型は `anyhow::Error` を使用する
- `main` の戻り値は `anyhow::Result<()>` とする
- ユーザーに見せるエラーメッセージは日本語・英語どちらでも可とするが、一貫性を保つ
- エラーは `stderr` に出力し、`exit code 1` で終了する

## テスト規約

- 単体テストはテスト対象モジュールの末尾に `#[cfg(test)]` ブロックで記述する
- 結合テストは `tests/` ディレクトリに配置する
- テスト関数名は `test_` プレフィックスを付ける（例: `test_parse_line_json`）
- `cargo test` がすべてパスする状態を維持する

## git規約

### ブランチ戦略

- `main`: リリースブランチ
- `feature/[機能名]`: 機能開発ブランチ
- `fix/[修正内容]`: バグ修正ブランチ

### コミットメッセージ

```
<type>: <概要>

<詳細（省略可）>
```

**type一覧:**

| type | 用途 |
|---|---|
| `feat` | 新機能追加 |
| `fix` | バグ修正 |
| `docs` | ドキュメントのみの変更 |
| `refactor` | リファクタリング |
| `test` | テストの追加・修正 |
| `chore` | ビルド設定・依存関係の変更 |

**例:**
```
feat: 折れ線グラフレンダラーを実装

plottersクレートを使用してJSONデータから折れ線グラフPNGを生成する。
```

## ビルド・品質チェック手順

```bash
# フォーマット
cargo fmt

# Lintチェック
cargo clippy -- -D warnings

# テスト
cargo test

# リリースビルド
cargo build --release
```

コード変更後は必ず上記を順に実行し、すべてパスすることを確認する。
