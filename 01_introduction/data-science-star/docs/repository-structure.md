# リポジトリ構造定義書

## フォルダ・ディレクトリ構成

```
data-science-star/
├── Cargo.toml               # プロジェクト設定・依存関係
├── Cargo.lock               # 依存関係ロックファイル
├── CLAUDE.md                # Claude Code向け開発ルール
├── .gitignore
├── data/                    # 入力データ（JSONファイル）
│   ├── users.json
│   └── friendship.json
├── docs/                    # 永続的ドキュメント
│   ├── product-requirements.md
│   ├── functional-design.md
│   ├── architecture.md
│   ├── repository-structure.md
│   ├── development-guidelines.md
│   └── glossary.md
├── .steering/               # 作業単位のドキュメント（一時的）
│   └── [YYYYMMDD]-[タイトル]/
│       ├── requirements.md
│       ├── design.md
│       └── tasklist.md
└── src/                     # ソースコード
    ├── main.rs              # エントリポイント
    ├── models.rs            # データ構造定義
    ├── loader.rs            # JSONファイル読み込み
    ├── network.rs           # ネットワーク操作
    └── stats.rs             # 統計算出
```

## ディレクトリの役割

| ディレクトリ | 役割 |
|------------|------|
| `data/` | アプリケーションが読み込む入力JSONファイルを配置 |
| `docs/` | プロジェクト全体の永続的な設計・仕様ドキュメント |
| `.steering/` | 作業単位の一時ドキュメント（作業完了後も履歴として保持） |
| `src/` | Rustソースコード |

## ファイル配置ルール

- ソースコードはすべて `src/` に配置する
- 入力データは `src/` に含めず `data/` に配置する
- ドキュメントは永続的なものは `docs/`、作業単位のものは `.steering/` に配置する
- `Cargo.toml` はプロジェクトルートに配置する
