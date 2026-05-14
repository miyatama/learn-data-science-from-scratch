# about

統計の学習。RustでCLIツールを実装。

+ [sample](https://github.com/joelgrus/data-science-from-scratch/blob/master/scratch/statistics.py)

共分散

```math
Sxy = \frac{1}{n}\sum_{i=1}^n(x_i - \bar{x})(y_i - \bar{y})
```

相関係数

```math
r = \frac{Sxy}{Sx Sy}
```

## Version 1.0.0

情報を元に下記を計算する

+ 代表値
  + 平均値
  + 中央値
  + 分位数
  + 最頻値
+ 散らばり
  + 範囲
  + 分散
  + 標準偏差
+ 相関
  + 共分散
  + 相関係数

## Version 1.0.1

ヘルプ機能を実装する。-h or --helpでパラメタやUsageを表示する。

## Version 1.0.2

下記変更を加える

+ 散らばりの範囲は下記を出力する
  + 最小値
  + 最大値
  + 数値の範囲(最大値 - 最小値)
+ 相関に下記を加える
  + 3σの範囲内で計算した共分散と相関係数
