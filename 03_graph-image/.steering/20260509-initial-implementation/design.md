# 初回実装 設計

## 実装アプローチ

`clap` derive APIでCLI引数を定義し、`serde_json` でJSONをデシリアライズ、`plotters` でPNG画像を生成する。
グラフタイプごとに独立したRendererモジュールを実装し、共通トレイトで統一的に呼び出す。

## モジュール構成

```
src/
├── main.rs          # CLI定義・エントリポイント
├── input.rs         # ファイル/標準入力の読み込み
├── parser.rs        # JSON → データ構造のパース
├── renderer/
│   ├── mod.rs       # Rendererトレイト・共通型
│   ├── line.rs      # LineRenderer
│   ├── bar.rs       # BarRenderer
│   └── scatter.rs   # ScatterRenderer
└── error.rs         # エラー型（anyhow利用のため最小限）
```

## 各モジュールの設計

### main.rs

```rust
#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "./output/image.png")]
    output: String,
    #[arg(short, long, default_value_t = 256)]
    size: u32,
    #[arg(short, long)]
    input: Option<String>,
    #[arg(short = 't', long = "type", default_value = "line")]
    graph_type: String,
}
```

処理フロー:
1. CLI引数パース
2. `InputReader::read()` でJSON文字列取得
3. `graph_type` に応じて対応する Renderer を生成・実行

### input.rs

```rust
pub fn read(input: Option<&str>) -> anyhow::Result<String>
```

- `input` が `Some(path)` → ファイル読み込み
- `input` が `None` → 標準入力から読み込み

### parser.rs

グラフタイプごとのデータ構造を定義し、JSON文字列からデシリアライズする。

```rust
#[derive(Deserialize)]
pub struct DataPoint { pub x: f64, pub y: f64 }

#[derive(Deserialize)]
pub struct Series<T> { pub label: Option<String>, pub data: Vec<T> }

#[derive(Deserialize)]
pub struct LineData {
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub series: Vec<Series<DataPoint>>,
}

#[derive(Deserialize)]
pub struct BarData {
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub categories: Vec<String>,
    pub series: Vec<Series<f64>>,
}

#[derive(Deserialize)]
pub struct ScatterData {
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub series: Vec<Series<DataPoint>>,
}
```

### renderer/mod.rs

```rust
pub trait Renderer {
    fn render(&self, output_path: &str, size: u32) -> anyhow::Result<()>;
}
```

### renderer/line.rs

- `LineData` を受け取り `plotters` で折れ線グラフを描画
- 複数系列を異なる色で描画
- X/Y軸の範囲はデータの min/max から自動計算

### renderer/bar.rs

- `BarData` を受け取り `plotters` で棒グラフを描画
- カテゴリ数に応じて棒の幅を自動調整
- 複数系列の場合は並列棒グラフ

### renderer/scatter.rs

- `ScatterData` を受け取り `plotters` で散布図を描画
- 複数系列を異なる色・マーカーで描画

## サンプルJSONファイル

### examples/line.json
```json
{
  "title": "Line Chart",
  "x_label": "X",
  "y_label": "Y",
  "series": [
    {
      "label": "series1",
      "data": [
        {"x": 1.0, "y": 2.0},
        {"x": 2.0, "y": 4.0},
        {"x": 3.0, "y": 3.0},
        {"x": 4.0, "y": 6.0}
      ]
    }
  ]
}
```

### examples/bar.json
```json
{
  "title": "Bar Chart",
  "x_label": "Category",
  "y_label": "Value",
  "categories": ["A", "B", "C", "D"],
  "series": [
    {
      "label": "series1",
      "data": [10.0, 25.0, 15.0, 30.0]
    }
  ]
}
```

### examples/scatter.json
```json
{
  "title": "Scatter Plot",
  "x_label": "X",
  "y_label": "Y",
  "series": [
    {
      "label": "series1",
      "data": [
        {"x": 1.0, "y": 2.0},
        {"x": 3.0, "y": 5.0},
        {"x": 2.0, "y": 1.0},
        {"x": 4.0, "y": 4.0}
      ]
    }
  ]
}
```

## 影響範囲の分析

- 新規プロジェクトのため既存コードへの影響なし
- `Cargo.toml` の新規作成が必要
- `output/` ディレクトリは `.gitignore` に追加済みであること確認が必要
