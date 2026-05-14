# 実装設計 - Version 1.0.0 初回実装

## 実装アプローチ

Rust のシングルバイナリ CLI として実装する。
モジュールを `statistics` と `correlation` に分割し、`main.rs` がエントリポイントとして各モジュールを呼び出す。

## 変更するコンポーネント

### 新規作成ファイル

| ファイル | 役割 |
|---------|------|
| `Cargo.toml` | パッケージ定義・serde/serde_json 依存 |
| `src/main.rs` | 引数解析・JSON読み込み・出力 |
| `src/statistics.rs` | 代表値・散らばり計算関数 + テスト |
| `src/correlation.rs` | 共分散・相関係数計算関数 + テスト |

## データ構造

```rust
// 入力
#[derive(serde::Deserialize)]
struct Input {
    datas: Vec<Vec<f64>>,
}

// main 内でのフロー
let input: Input = serde_json::from_str(&content)?;
let x = &input.datas[0];
let y = &input.datas[1];  // 相関用
```

## 関数一覧

### src/statistics.rs

```rust
pub fn mean(data: &[f64]) -> f64
pub fn median(data: &[f64]) -> f64          // 内部でソートした Vec を使用
pub fn quantile(data: &[f64], p: f64) -> f64
pub fn mode(data: &[f64]) -> f64            // i64 キャストで頻度カウント
pub fn data_range(data: &[f64]) -> f64
pub fn variance(data: &[f64]) -> f64        // 不偏分散 (n-1)
pub fn std_deviation(data: &[f64]) -> f64
```

### src/correlation.rs

```rust
pub fn covariance(x: &[f64], y: &[f64]) -> f64
pub fn correlation(x: &[f64], y: &[f64]) -> f64
```

## main.rs の処理フロー

```
1. std::env::args() でファイルパスを取得
2. fs::read_to_string() でファイル読み込み
3. serde_json::from_str() で JSON パース
4. datas[0] に対して代表値・散らばりを計算・表示
5. datas の長さが 2 以上なら datas[0] と datas[1] で相関を計算・表示
6. エラーは eprintln! + process::exit(1)
```

## 影響範囲の分析

- 新規プロジェクトのため既存コードへの影響なし
- `examples/statistics.json` は既存ファイルをそのまま使用
