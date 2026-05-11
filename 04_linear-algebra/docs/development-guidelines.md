# 開発ガイドライン

## コーディング規約

- Rustの標準的なイディオムに従う
- 関数は単一責任の原則に従い、1つの演算のみを担う
- エラーは `Result<T, String>` で返し、パニックしない
- `unwrap()` は使用しない（テストコードは除く）

## 命名規則

| 対象 | 規則 | 例 |
|------|------|----|
| 関数名 | snake_case | `vector_add`, `dot_product` |
| 変数名 | snake_case | `scalar`, `v1`, `v2` |
| モジュール名 | snake_case | `vector` |
| 定数 | SCREAMING_SNAKE_CASE | `MAX_DIMENSION` |

## テスト規約

- 各関数に対してユニットテストを `src/vector.rs` 内の `#[cfg(test)]` ブロックに記述する
- 正常系・異常系（次元不一致）の両方をテストする
- テスト関数名は `test_[関数名]_[ケース]` の形式とする

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_add_normal() { ... }

    #[test]
    fn test_vector_add_dimension_mismatch() { ... }
}
```

## git規約

- ブランチ名: `feature/[機能名]`
- コミットメッセージ: `feat:`, `fix:`, `docs:`, `test:` のプレフィックスを使用
- PRは機能単位で作成する
