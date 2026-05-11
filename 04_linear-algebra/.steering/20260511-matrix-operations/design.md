# 実装設計 — Version 2.0.0 行列操作

## 実装アプローチ

1. `src/matrix.rs` を新規作成し、行列演算の純粋関数を実装する
2. `src/main.rs` に `matrix` サブコマンドを追加する
3. `Cargo.toml` のバージョンを `2.0.0` に更新する

## 変更するコンポーネント

| ファイル | 変更内容 |
|----------|---------|
| `Cargo.toml` | バージョンを `2.0.0` に更新 |
| `src/matrix.rs` | 新規作成。行列演算モジュール |
| `src/main.rs` | `matrix` サブコマンドの追加 |

## データ構造

行列は `Vec<Vec<f64>>` で表現する。各内側 `Vec<f64>` が1行を表す。

```
Matrix = Vec<Vec<f64>>
例: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]  → 2行3列
```

## 関数設計

```rust
// src/matrix.rs

// shape: (行数, 列数) を返す
// 各行の列数が一致しない場合は Err を返す
pub fn shape(matrix: &[Vec<f64>]) -> Result<(usize, usize), String>
```

## 実装詳細

### `src/matrix.rs`

```rust
pub fn shape(matrix: &[Vec<f64>]) -> Result<(usize, usize), String> {
    if matrix.is_empty() {
        return Ok((0, 0));
    }
    let num_cols = matrix[0].len();
    for row in matrix.iter().skip(1) {
        if row.len() != num_cols {
            return Err("all rows must have the same number of columns".to_string());
        }
    }
    Ok((matrix.len(), num_cols))
}
```

### `src/main.rs` — 行列パース

```rust
// "1,2,3;4,5,6" → Vec<Vec<f64>>
// ';' で行を分割し、',' で列を分割して f64 にパース
fn parse_matrix(s: &str) -> Result<Vec<Vec<f64>>, String>
```

### CLIインターフェース

```
# shape
$ cargo run -- matrix shape "1,2,3;4,5,6"
(2, 3)

# ヘルプ
$ cargo run -- matrix --help
$ cargo run -- matrix shape --help
```

### ヘルプテキスト

**`matrix --help`:**
```
linear-algebra-matrix
Matrix operations

USAGE:
    linear-algebra matrix <SUBCOMMAND>

SUBCOMMANDS:
    shape  Get the shape (rows, cols) of a matrix

OPTIONS:
    -h, --help  Print help information
```

**`matrix shape --help`:**
```
linear-algebra-matrix-shape
Get the shape (rows, cols) of a matrix

USAGE:
    linear-algebra matrix shape <matrix>

ARGS:
    <matrix>  Matrix (rows separated by ';', cols by ',', e.g. "1,2,3;4,5,6")

OPTIONS:
    -h, --help  Print help information
```

## 影響範囲の分析

- `src/vector.rs` は変更なし（既存テスト継続通過）
- `src/main.rs` に `matrix` サブコマンドのブランチを追加
- `docs/functional-design.md` を更新（行列モジュールの追加）
