# 実装設計 — Version 1.0.0 初回実装

## 実装アプローチ

1. `Cargo.toml` でプロジェクトを初期化
2. `src/vector.rs` にベクトル演算の純粋関数を実装
3. `src/main.rs` にCLIのサブコマンドパースと呼び出しを実装
4. `src/vector.rs` 内の `#[cfg(test)]` ブロックにユニットテストを実装

## 変更するコンポーネント

| ファイル | 変更内容 |
|----------|---------|
| `Cargo.toml` | 新規作成。クレート名・エディション設定 |
| `src/main.rs` | 新規作成。CLIエントリーポイント |
| `src/vector.rs` | 新規作成。ベクトル演算モジュール |

## データ構造

```rust
// ベクトルはスライス参照で受け取り、Vec<f64> で返す
fn vector_add(v1: &[f64], v2: &[f64]) -> Result<Vec<f64>, String>
fn vector_subtract(v1: &[f64], v2: &[f64]) -> Result<Vec<f64>, String>
fn scalar_multiply(scalar: f64, v: &[f64]) -> Vec<f64>
fn dot_product(v1: &[f64], v2: &[f64]) -> Result<f64, String>
fn sum_of_squares(v: &[f64]) -> f64
fn magnitude(v: &[f64]) -> f64
```

## 実装詳細

### `src/vector.rs`

```rust
// 次元チェックのヘルパー
fn check_same_length(v1: &[f64], v2: &[f64]) -> Result<(), String> {
    if v1.len() != v2.len() {
        Err("vectors must have the same length".to_string())
    } else {
        Ok(())
    }
}

// vector_add: zip して各要素を加算
// vector_subtract: zip して各要素を減算
// scalar_multiply: map して各要素にスカラーを乗算
// dot_product: zip して積を取り sum
// sum_of_squares: map で二乗して sum
// magnitude: sum_of_squares の平方根（f64::sqrt）
```

### `src/main.rs`

```rust
// std::env::args() で引数を取得
// args[1] がサブコマンド
// ベクトル引数は "1,2,3" 形式をカンマ分割して f64 にパース
// 結果を println! で表示
// エラーは eprintln! で表示し std::process::exit(1)
```

## 影響範囲の分析

- 新規プロジェクトのため既存コードへの影響なし
- 全ファイルを新規作成する
