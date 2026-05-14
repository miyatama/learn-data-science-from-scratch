# ユビキタス言語定義

## ドメイン用語

| 日本語 | 英語 | コード上の名称 | 定義 |
|--------|------|---------------|------|
| 平均値 | Mean | `mean` | データの算術平均 |
| 中央値 | Median | `median` | ソート後の中央の値 |
| 分位数 | Quantile | `quantile` | データを順位で分割したときの境界値 |
| 25%分位数 | 1st Quartile (Q1) | `quantile_25` | 下位25%の境界値 |
| 75%分位数 | 3rd Quartile (Q3) | `quantile_75` | 下位75%の境界値 |
| 最頻値 | Mode | `mode` | 最も多く出現する値 |
| 範囲 | Range | `data_range` | 最大値 − 最小値 |
| 分散 | Variance | `variance` | 平均からの偏差の二乗平均（不偏） |
| 標準偏差 | Standard Deviation | `std_deviation` | 分散の正の平方根 |
| 共分散 | Covariance | `covariance` | 2変数の偏差の積の平均（標本） |
| 相関係数 | Correlation Coefficient | `correlation` | 共分散を両標準偏差で正規化した値（−1〜1） |
| 代表値 | Central Tendency | `CentralTendency` | データの中心的傾向を表す統計量 |
| 散らばり | Dispersion | `Dispersion` | データのばらつきを表す統計量 |
| データ列 | Data Series | `data` / `datas[i]` | 計算対象となる f64 の配列 |

## 数式と記号

| 記号 | 意味 |
|------|------|
| $n$ | データ数 |
| $x_i$ | i 番目のデータ値 |
| $\bar{x}$ | 平均値 |
| $s^2$ | 不偏分散 |
| $s$ | 標準偏差 |
| $S_{xy}$ | 共分散 |
| $r$ | ピアソン相関係数 |

## 英語・日本語対応表（出力文言）

| 出力文言（日本語） | 変数/関数名 |
|------------------|-------------|
| 平均値 | `mean` |
| 中央値 | `median` |
| 25%分位数 | `quantile_25` |
| 75%分位数 | `quantile_75` |
| 最頻値 | `mode` |
| 範囲 | `data_range` |
| 分散 | `variance` |
| 標準偏差 | `std_deviation` |
| 共分散 | `covariance` |
| 相関係数 | `correlation` |
