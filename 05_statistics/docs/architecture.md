# 技術仕様書

## テクノロジースタック

| 分類 | 技術 | バージョン |
|------|------|-----------|
| 言語 | Rust | 1.70 以上 |
| ビルドツール | Cargo | （Rust 同梱） |
| JSON パース | serde / serde_json | 1.x |

## 開発ツールと手法

| ツール | 用途 |
|--------|------|
| `cargo build --release` | リリースビルド |
| `cargo test` | 単体テスト実行 |
| `cargo clippy` | 静的解析・リント |
| `cargo fmt` | コードフォーマット |

## 依存クレート

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

外部ネットワーク通信・データベース・非同期処理は使用しない。

## 技術的制約と要件

- **unsafe コード禁止**：学習目的のため安全な Rust のみ使用
- **panic! 禁止**：エラーは `Result` / `eprintln!` + `process::exit(1)` で処理
- **標準出力**：計算結果は `println!` で stdout に出力
- **エラー出力**：エラーメッセージは `eprintln!` で stderr に出力
- **整数演算禁止**：統計計算はすべて `f64` で行う

## パフォーマンス要件

| 条件 | 目標 |
|------|------|
| 要素数 1,000 以下のデータセット | 1 秒以内に結果出力 |
| メモリ使用量 | データサイズに比例、特段の上限なし（学習用） |

## ビルドターゲット

```
cargo build --release
```

生成バイナリ: `./target/release/statistics`
