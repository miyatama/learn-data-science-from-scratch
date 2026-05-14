# design.md

## 実装アプローチ

既存の `line`, `bar`, `scatter` と同じパターンで `histogram` を追加する。

### 処理フロー

1. JSONをパース → `HistogramData` 構造体に変換
2. 生データ配列を `size`（ビン幅）で集計し、各ビンの頻度（count）を算出
3. ビンを棒として描画。x軸目盛りは `x_scale` 間隔で打つ

### ビン集計ロジック

```
bin_index = floor(value / size)
bin_start = bin_index * size
```

各ビンのx座標は `bin_start`、y値はそのビンに入ったデータ数（count）。

## 変更するコンポーネント

### 1. `src/parser.rs` に追加

```rust
#[derive(Debug, Deserialize)]
pub struct HistogramData {
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub size: f64,       // ビン幅
    pub x_scale: f64,    // x軸目盛り間隔
    pub series: Vec<Series<f64>>,
}

pub fn parse_histogram(json: &str) -> anyhow::Result<HistogramData>
```

### 2. `src/renderer/histogram.rs` を新規作成

```rust
pub struct HistogramRenderer {
    pub data: HistogramData,
}

impl Renderer for HistogramRenderer {
    fn render(&self, output_path: &str, size: u32) -> anyhow::Result<()>
}
```

- 各seriesのデータをビン集計
- plottersで棒グラフとして描画
- x軸目盛りは `x_scale` 間隔

### 3. `src/renderer/mod.rs` に追加

```rust
pub mod histogram;
```

### 4. `src/main.rs` に追加

```rust
"histogram" => Box::new(HistogramRenderer {
    data: parser::parse_histogram(&json)?,
}),
```

## 影響範囲の分析

- 既存コード（`line`, `bar`, `scatter`）の変更なし
- `parser.rs` に新しい構造体と関数を追加するのみ
- `main.rs` の match 文に1アームを追加するのみ
