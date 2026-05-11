# リポジトリ構造定義書

## ディレクトリ構成

```
04_linear-algebra/
├── CLAUDE.md                  # Claude Code向け開発ルール
├── README.md                  # プロジェクト概要・バージョン履歴
├── Cargo.toml                 # Rustプロジェクト設定・依存関係
├── docs/                      # 永続的ドキュメント
│   ├── product-requirements.md
│   ├── functional-design.md
│   ├── architecture.md
│   ├── repository-structure.md
│   ├── development-guidelines.md
│   └── glossary.md
├── .steering/                 # 作業単位のステアリングドキュメント
│   └── [YYYYMMDD]-[タイトル]/
│       ├── requirements.md
│       ├── design.md
│       └── tasklist.md
└── src/
    ├── main.rs                # CLIエントリーポイント
    └── vector.rs              # ベクトル演算モジュール
```

## ディレクトリの役割

| ディレクトリ/ファイル | 役割 |
|----------------------|------|
| `docs/` | プロジェクト全体の永続的な設計ドキュメント |
| `.steering/` | 作業単位の一時ドキュメント（完了後も履歴として保持） |
| `src/` | Rustソースコード |
| `src/main.rs` | CLIエントリーポイント、引数パース・結果表示 |
| `src/vector.rs` | ベクトル演算の純粋関数群 |
| `Cargo.toml` | クレート名・バージョン・依存関係の定義 |

## ファイル配置ルール

- ソースコードは `src/` 以下に配置する
- 新しい演算カテゴリを追加する場合は `src/` に新モジュールファイルを追加する
- ドキュメントは `docs/`（永続）または `.steering/`（一時）に配置し、`src/` には置かない
