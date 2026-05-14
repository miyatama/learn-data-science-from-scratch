# 実装設計 - Version 1.0.2

## 実装アプローチ

`src/statistics.rs` に `min` / `max` 関数を追加し、`src/main.rs` の出力を拡張する。
3σ フィルタは `main.rs` 内でインライン実装し、フィルタ後のスライスを相関関数に渡す。

## 変更するコンポーネント

| ファイル | 変更内容 |
|---------|---------|
| `Cargo.toml` | version を `1.0.2` に更新 |
| `src/statistics.rs` | `min` / `max` 関数を追加、テストを追加 |
| `src/main.rs` | 散らばり出力を3項目に拡張、3σフィルタ後の相関を追加 |

変更なし: `src/correlation.rs`

## 関数追加（src/statistics.rs）

```rust
pub fn min(data: &[f64]) -> f64  // data.iter().cloned().fold(f64::INFINITY, f64::min)
pub fn max(data: &[f64]) -> f64  // data.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
```

既存の `data_range` は `max - min` を使うよう内部を整理する。

## 出力変更（src/main.rs）

### 散らばりセクション（変更後）

```
=== 散らばり (datas[0]) ===
最小値      : <value>
最大値      : <value>
数値の範囲  : <value>
分散        : <value>
標準偏差    : <value>
```

### 相関セクション（変更後）

```
=== 相関 (datas[0] vs datas[1]) ===
共分散      : <value>
相関係数    : <value>
--- 3σ範囲内 ---
共分散      : <value>
相関係数    : <value>
```

## 3σフィルタの実装

```rust
let m = statistics::mean(x);
let s = statistics::std_deviation(x);
let lower = m - 3.0 * s;
let upper = m + 3.0 * s;

// x と y を同じインデックスでフィルタ
let (x3, y3): (Vec<f64>, Vec<f64>) = x_trim
    .iter()
    .zip(y_trim.iter())
    .filter(|(&xi, _)| xi >= lower && xi <= upper)
    .map(|(&xi, &yi)| (xi, yi))
    .unzip();
```

## 影響範囲の分析

- `correlation.rs` への影響なし
- ヘルプ機能（`print_help`）は更新不要（出力フォーマットの変更はヘルプ対象外）
- 既存テストへの影響なし（`data_range` の動作は変わらない）
