# リポジトリ構造定義書

## フォルダ・ディレクトリ構成

```
03_graph-image/
├── .steering/                        # 作業単位の一時ドキュメント
│   └── YYYYMMDD-[タイトル]/
│       ├── requirements.md
│       ├── design.md
│       └── tasklist.md
├── docs/                             # 永続的ドキュメント
│   ├── product-requirements.md
│   ├── functional-design.md
│   ├── architecture.md
│   ├── repository-structure.md       # 本ファイル
│   ├── development-guidelines.md
│   └── glossary.md
├── src/                              # Rustソースコード
│   ├── main.rs                       # エントリポイント・CLIパース
│   ├── input.rs                      # InputReader（ファイル/標準入力）
│   ├── parser.rs                     # JsonParser・データ構造定義
│   ├── renderer/
│   │   ├── mod.rs                    # Rendererトレイト定義
│   │   ├── line.rs                   # 折れ線グラフRenderer
│   │   ├── bar.rs                    # 棒グラフRenderer
│   │   └── scatter.rs               # 散布図Renderer
│   └── error.rs                      # エラー型定義
├── tests/                            # 結合テスト
│   └── integration_test.rs
├── examples/                         # サンプルJSONファイル
│   ├── line.json
│   ├── bar.json
│   └── scatter.json
├── output/                           # デフォルト出力先（gitignore対象）
├── Cargo.toml
├── Cargo.lock
├── CLAUDE.md
├── README.md
└── .gitignore
```

## ディレクトリの役割

| ディレクトリ | 役割 |
|---|---|
| `.steering/` | 作業単位の要求・設計・タスクドキュメント。作業完了後も履歴として保持 |
| `docs/` | プロダクト全体の永続的な設計・仕様ドキュメント |
| `src/` | Rustソースコード本体 |
| `src/renderer/` | グラフタイプごとの描画モジュール |
| `tests/` | `cargo test` で実行される結合テスト |
| `examples/` | 動作確認・デモ用のサンプルJSONファイル |
| `output/` | デフォルト出力先。gitignore対象 |

## ファイル配置ルール

- Rustソースは `src/` 直下またはサブモジュール (`src/renderer/`) に配置する
- グラフタイプを追加する場合は `src/renderer/` に新ファイルを追加し、`mod.rs` でエクスポートする
- サンプルデータは `examples/` に配置し、ファイル名はグラフタイプ名に合わせる
- 生成画像は `output/` に出力する。このディレクトリはgitignoreで管理対象外とする
- ドキュメントは `docs/`（永続） または `.steering/`（作業単位）のいずれかに配置し、ルート直下に散在させない
