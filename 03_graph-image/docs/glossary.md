# ユビキタス言語定義

## ドメイン用語

| 用語（日本語） | 用語（英語） | 定義 |
|---|---|---|
| グラフ | Graph | データを視覚的に表現した図。折れ線・棒・散布図を指す |
| 折れ線グラフ | Line Chart | X/Y座標の点を線で結んだグラフ。時系列データの可視化に使用 |
| 棒グラフ | Bar Chart | カテゴリごとの値を棒の高さで表現したグラフ |
| 散布図 | Scatter Plot | X/Y座標の点を打点のみで表現したグラフ。相関の確認に使用 |
| 系列 | Series | 1つのグラフ上に表示されるデータのまとまり。複数系列を重ねて描画可能 |
| カテゴリ | Category | 棒グラフのX軸に配置されるラベル付きの分類 |
| データポイント | Data Point | グラフ上の1つの点。X値とY値のペアで表現される |
| 出力画像 | Output Image | レンダリング結果として生成されるPNGファイル |
| 画像サイズ | Image Size | 出力画像の一辺のピクセル数（正方形）。`--size` オプションで指定 |

## コード上の命名規則

| 概念 | コード上の名前 | 型・種別 |
|---|---|---|
| グラフタイプ | `GraphType` | Enum (`Line`, `Bar`, `Scatter`) |
| 折れ線グラフデータ | `LineData` | struct |
| 棒グラフデータ | `BarData` | struct |
| 散布図データ | `ScatterData` | struct |
| 系列 | `Series` | struct |
| データポイント | `DataPoint` | struct (`x: f64`, `y: f64`) |
| 描画トレイト | `Renderer` | trait |
| 折れ線Renderer | `LineRenderer` | struct |
| 棒グラフRenderer | `BarRenderer` | struct |
| 散布図Renderer | `ScatterRenderer` | struct |
| 出力パス | `output_path` | `&str` / `String` |
| 画像サイズ | `size` | `u32` |

## CLIオプション対応表

| オプション | 短縮形 | コード上の変数名 |
|---|---|---|
| `--output` | `-o` | `output` |
| `--size` | `-s` | `size` |
| `--input` | `-i` | `input` |
| `--type` | `-t` | `graph_type` |
