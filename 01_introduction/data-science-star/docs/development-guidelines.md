# 開発ガイドライン

## コーディング規約

- Rust公式スタイルガイドに従う
- `cargo fmt` を実行してフォーマットを統一する
- `cargo clippy` の警告をすべて解消してからコミットする
- `unwrap()` は使用しない。`expect()` またはエラーハンドリングを使用する
- `pub` は必要最小限にとどめる

## 命名規則

| 対象 | 規則 | 例 |
|------|------|----|
| 関数・変数 | snake_case | `number_of_friends` |
| 型・構造体・列挙型 | PascalCase | `FriendMap`, `User` |
| 定数 | SCREAMING_SNAKE_CASE | `MAX_USERS` |
| モジュール | snake_case | `network`, `stats` |
| ファイル名 | snake_case | `loader.rs` |

## テスト規約

- 各モジュールにユニットテストを `#[cfg(test)]` ブロックで記述する
- テスト関数名は `test_` プレフィックスを付ける（例: `test_number_of_friends`）
- テストデータは実データ（`data/`）に依存せず、テスト内でインラインで定義する

## git規約

### ブランチ命名
- `feature/[機能名]` - 新機能
- `fix/[修正内容]` - バグ修正
- `docs/[ドキュメント名]` - ドキュメントのみの変更

### コミットメッセージ
```
[種別] 変更内容の概要

# 種別
feat:  新機能
fix:   バグ修正
docs:  ドキュメントのみの変更
style: コードの動作に影響しない変更（フォーマット等）
test:  テストの追加・修正
```

### 例
```
feat: 次数中心性の算出機能を追加
docs: functional-design.mdにモジュール構成を追記
```
