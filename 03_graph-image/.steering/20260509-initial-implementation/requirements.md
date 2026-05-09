# 初回実装 要求事項

## 概要

JSONを入力として受け取り、折れ線・棒グラフ・散布図をPNG画像として出力するRust製CLIツールの初回実装。

## 実装する機能

### CLIインターフェース

- `--output` / `-o` : 出力先ファイルパス（デフォルト: `./output/image.png`）
- `--size` / `-s` : 出力画像の一辺ピクセル数（デフォルト: `256`）
- `--input` / `-i` : 入力JSONファイルパス（省略時は標準入力）
- `--type` / `-t` : グラフタイプ `line` / `bar` / `scatter`（デフォルト: `line`）

### グラフタイプ

1. **折れ線グラフ (line)**: X/Y座標の点を線で結ぶ。複数系列対応
2. **棒グラフ (bar)**: カテゴリ別の値を棒で表現。複数系列対応
3. **散布図 (scatter)**: X/Y座標の点を打点で表現。複数系列対応

### 入力

- JSONファイルまたは標準入力からJSONを読み込む
- 各グラフタイプのJSONフォーマットは `docs/functional-design.md` 参照

### 出力

- PNG形式の画像ファイルを指定パスに保存
- 出力先ディレクトリが存在しない場合は自動作成

### エラー処理

- 入力ファイルが存在しない場合: stderr出力 + exit code 1
- JSONパースエラー: stderr出力 + exit code 1
- 不正なグラフタイプ: stderr出力 + exit code 1
- 出力先書き込みエラー: stderr出力 + exit code 1

## ユーザーストーリー

- JSONファイルを `--input` で渡し、折れ線グラフPNGを得る
- 標準入力からJSONをパイプして棒グラフPNGを得る
- `--size 512` で大きめのグラフ画像を生成する

## 受け入れ基準

- [ ] `cargo build --release` でビルドが成功する
- [ ] `cargo test` がすべてパスする
- [ ] `cargo clippy -- -D warnings` で警告ゼロ
- [ ] `examples/line.json` を入力として折れ線グラフPNGが生成される
- [ ] `examples/bar.json` を入力として棒グラフPNGが生成される
- [ ] `examples/scatter.json` を入力として散布図PNGが生成される
- [ ] 標準入力からJSONを渡して画像が生成される
- [ ] 存在しない出力ディレクトリが自動作成される

## 制約事項

- Rust stable チャンネルで動作すること
- 使用クレートは `docs/architecture.md` 記載のものに限定する
