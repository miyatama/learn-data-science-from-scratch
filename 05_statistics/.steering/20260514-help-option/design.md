# 実装設計 - Version 1.0.1 ヘルプ機能実装

## 実装アプローチ

`src/main.rs` の引数解析部分に `-h` / `--help` の判定を追加する。
外部クレートは追加せず、`std::env::args()` で判定する。

## 変更するコンポーネント

| ファイル | 変更内容 |
|---------|---------|
| `src/main.rs` | 引数に `-h` / `--help` が含まれる場合、ヘルプを表示して `process::exit(0)` |
| `Cargo.toml` | version を `1.0.1` に更新 |

変更なし: `src/statistics.rs` / `src/correlation.rs`

## ヘルプ出力内容

```
統計計算 CLI - Data Science From Scratch Chapter 5

Usage:
    statistics <input.json>
    statistics -h | --help

Options:
    <input.json>    統計計算対象の JSON ファイルパス
    -h, --help      このヘルプを表示して終了

Input JSON Format:
    {
      "datas": [
        [<f64>, ...],   // datas[0]: 代表値・散らばりの計算対象
        [<f64>, ...]    // datas[1]: 相関計算の対象（省略可）
      ]
    }

Output:
    代表値（平均値・中央値・25%分位数・75%分位数・最頻値）
    散らばり（範囲・分散・標準偏差）
    相関（共分散・相関係数）※ datas[1] がある場合のみ
```

## 処理フロー変更箇所

```rust
// 変更前
if args.len() < 2 {
    eprintln!("Usage: {} <input.json>", args[0]);
    process::exit(1);
}

// 変更後
if args.len() < 2 || args.iter().any(|a| a == "-h" || a == "--help") {
    if args.len() < 2 {
        eprintln!("Usage: {} <input.json>", args[0]);
        process::exit(1);
    }
    print_help();
    process::exit(0);
}
```

実際には引数チェックを整理して可読性を高める。

## 影響範囲の分析

- `statistics.rs` / `correlation.rs` への影響なし
- 既存の JSON 読み込み・計算処理への影響なし
