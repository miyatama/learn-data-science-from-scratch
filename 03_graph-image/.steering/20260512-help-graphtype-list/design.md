# design.md

## 実装アプローチ

`src/main.rs` の `Cli` 構造体における `graph_type` フィールドの `#[arg]` アトリビュートに
`help` パラメータを追加し、指定できるグラフタイプを明示する。

clapの `help` オプションを使うことで、`--help` 実行時に説明文が表示される。

## 変更するコンポーネント

### `src/main.rs` のみ変更

```rust
// 変更前
#[arg(short = 't', long = "type", default_value = "line")]
graph_type: String,

// 変更後
#[arg(short = 't', long = "type", default_value = "line",
      help = "Graph type [possible values: line, bar, scatter, histogram]")]
graph_type: String,
```

## 影響範囲の分析

- `main.rs` の1行変更のみ
- 動作ロジックへの影響なし
