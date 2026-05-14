# requirements.md

## 概要

Version 1.1.0 対応: ヒストグラム機能の追加

## 変更・追加する機能の説明

`--type histogram` オプションを追加し、生データの配列を受け取ってヒストグラムを生成・出力する。

## ユーザーストーリー

- ユーザーとして、生データの配列を含むJSONを渡し、`--type histogram` を指定することで、ヒストグラム画像を得たい。

## 受け入れ基準

1. `--type histogram` を指定した場合にヒストグラムを出力できる
2. `examples/histgram.json` を入力として正常に動作する
3. JSONフォーマットの仕様を満たす:
   - `size`: データをどの区間（ビン幅）で集計するか（float）
   - `x_scale`: x軸の目盛りを打つ間隔（float）
   - `series[].data`: 生データの配列（float配列）
   - `title`, `x_label`, `y_label`: 任意のラベル（オプション）
4. 既存の `line`, `bar`, `scatter` グラフ型に影響を与えない
5. ビルド・テストが通ること

## 制約事項

- 言語: Rust（既存プロジェクトに追加）
- 描画ライブラリ: plotters（既存）
- JSONパース: serde_json（既存）
