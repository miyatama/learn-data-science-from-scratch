# リポジトリ構造定義書

## フォルダ・ディレクトリ構成

```
05_statistics/
├── Cargo.toml                        # パッケージ定義・依存クレート
├── Cargo.lock                        # 依存バージョンロック
├── README.md                         # プロジェクト概要
├── CLAUDE.md                         # Claude Code 向け開発ルール
│
├── src/
│   ├── main.rs                       # エントリポイント（引数解析・出力）
│   ├── statistics.rs                 # 代表値・散らばり計算
│   └── correlation.rs                # 共分散・相関係数計算
│
├── examples/
│   └── statistics.json               # サンプル入力データ
│
├── docs/                             # 永続的ドキュメント
│   ├── product-requirements.md
│   ├── functional-design.md
│   ├── architecture.md
│   ├── repository-structure.md
│   ├── development-guidelines.md
│   └── glossary.md
│
└── .steering/                        # 作業単位ドキュメント（一時）
    └── 20260514-initial-implementation/
        ├── requirements.md
        ├── design.md
        └── tasklist.md
```

## ディレクトリの役割

| ディレクトリ | 役割 |
|-------------|------|
| `src/` | Rust ソースコード |
| `examples/` | 動作確認用サンプル入力ファイル |
| `docs/` | 恒久的な設計・仕様ドキュメント |
| `.steering/` | 作業単位の一時ドキュメント |

## ファイル配置ルール

- ソースコードは必ず `src/` 直下に配置する（サブディレクトリは作らない）
- サンプルデータは `examples/` に配置し、ファイル名は用途を示す名称にする
- ドキュメントは `docs/`（恒久）か `.steering/`（一時）のいずれかに配置し、混在させない
- バイナリ・ビルド成果物は `target/` に自動生成されるため手動管理しない
