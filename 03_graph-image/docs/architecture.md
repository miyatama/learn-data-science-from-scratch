# 技術仕様書

## テクノロジースタック

| 種別 | 技術 | 用途 |
|---|---|---|
| 言語 | Rust (edition 2021) | メイン実装言語 |
| CLIパース | `clap` (v4) | コマンドライン引数のパース |
| JSONパース | `serde` + `serde_json` | JSON入力のデシリアライズ |
| グラフ描画 | `plotters` | グラフのレンダリング・PNG出力 |
| エラーハンドリング | `anyhow` | エラー伝播の簡素化 |

## 開発ツールと手法

| ツール | 用途 |
|---|---|
| `cargo` | ビルド・テスト・依存管理 |
| `rustfmt` | コードフォーマット |
| `clippy` | Lintチェック |

## ディレクトリ構成（概略）

```
03_graph-image/
├── src/
│   ├── main.rs          # エントリポイント・CLIパース
│   ├── input.rs         # InputReader
│   ├── parser.rs        # JsonParser・データ構造定義
│   ├── renderer/
│   │   ├── mod.rs       # Rendererトレイト定義
│   │   ├── line.rs      # LineRenderer
│   │   ├── bar.rs       # BarRenderer
│   │   └── scatter.rs   # ScatterRenderer
│   └── error.rs         # エラー型定義
├── Cargo.toml
└── Cargo.lock
```

## 技術的制約と要件

- Rust stable チャンネルで動作すること
- `cargo build --release` でシングルバイナリを生成できること
- 外部プロセス・ネットワーク接続不要（完全オフライン動作）

## パフォーマンス要件

- 通常用途（数百〜数千点のデータ）で数秒以内に画像生成が完了すること
- メモリ使用量は入力データサイズに比例し、過大なアロケーションを行わないこと

## Cargo.toml 依存関係

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
plotters = "0.3"
anyhow = "1"
```
