# 要求事項 - Version 1.0.0 初回実装

## 変更・追加する機能の説明

`./examples/statistics.json` を入力として受け取り、以下の統計量を計算・表示する Rust CLI ツールを新規実装する。

### 代表値

- 平均値（mean）
- 中央値（median）
- 分位数（quantile）：25% / 75%
- 最頻値（mode）

### 散らばり

- 範囲（data_range）：最大値 − 最小値
- 分散（variance）：不偏分散
- 標準偏差（std_deviation）

### 相関

- 共分散（covariance）
- 相関係数（correlation）：ピアソン

## ユーザーストーリー

```
As a データサイエンス学習者
I want to  JSON ファイルを CLI に渡す
So that    代表値・散らばり・相関係数を一覧で確認できる
```

## 受け入れ基準

- [ ] `cargo build --release` でビルドが成功する
- [ ] `./target/release/statistics ./examples/statistics.json` で実行できる
- [ ] 代表値（平均値・中央値・25%分位数・75%分位数・最頻値）が出力される
- [ ] 散らばり（範囲・分散・標準偏差）が出力される
- [ ] 相関（共分散・相関係数）が出力される
- [ ] `cargo clippy` で警告がゼロ
- [ ] `cargo test` が全テストパス

## 制約事項

- 外部クレートは `serde` / `serde_json` のみ使用可
- `unsafe` コード禁止
- `unwrap()` はプロダクションコードで使用禁止
