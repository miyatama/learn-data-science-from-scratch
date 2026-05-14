# 開発ガイドライン

## コーディング規約

- `cargo fmt` のデフォルト設定に準拠する
- `cargo clippy` の警告はすべて解消する
- `unsafe` ブロックは使用しない
- `unwrap()` / `expect()` はテストコード内のみ許可。プロダクションコードでは `?` 演算子か明示的なエラーハンドリングを使用する
- パニックを起こす可能性のある操作（スライスの直接インデックスアクセス等）は `.get()` で安全に行う

## 命名規則

| 対象 | 規則 | 例 |
|------|------|----|
| 関数・変数 | snake_case | `fn mean(data: &[f64])` |
| 構造体・型 | PascalCase | `struct CentralTendency` |
| 定数 | SCREAMING_SNAKE_CASE | `const MAX_SIZE: usize` |
| モジュール | snake_case | `mod statistics` |

## エラー処理規約

```rust
// OK: Result を返して ? で伝播
fn load_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

// OK: main でエラーを受け取り stderr に出力して終了
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

// NG: unwrap をプロダクションコードで使用
let content = std::fs::read_to_string(path).unwrap();
```

## テスト規約

- 各統計関数に対して `#[cfg(test)]` モジュール内に単体テストを記述する
- テストデータは既知の値（手計算可能なもの）を使用する
- テスト関数名は `test_<関数名>_<条件>` の形式にする

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_basic() {
        assert_eq!(mean(&[1.0, 2.0, 3.0]), 2.0);
    }
}
```

## git 規約

### ブランチ命名

```
feature/<機能名>   # 新機能
fix/<バグ内容>     # バグ修正
```

### コミットメッセージ

```
<type>: <概要>（日本語可）

feat: Version 1.0.0 の統計計算 CLI を実装
fix: 中央値計算の偶数長データ対応
```

| type | 用途 |
|------|------|
| feat | 新機能追加 |
| fix | バグ修正 |
| docs | ドキュメントのみの変更 |
| refactor | 機能変更を伴わないリファクタリング |
| test | テスト追加・修正 |
