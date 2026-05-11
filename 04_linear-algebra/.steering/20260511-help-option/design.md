# 実装設計 — Version 1.0.1 ヘルプオプション

## 実装アプローチ

`src/main.rs` のみ変更する。`src/vector.rs` は変更なし。

引数パース時に `-h` / `--help` を検出し、対応するヘルプテキストを表示して終了する。

## 変更するコンポーネント

| ファイル | 変更内容 |
|----------|---------|
| `src/main.rs` | ヘルプ表示関数の追加、引数パース処理の更新 |
| `Cargo.toml` | バージョンを 1.0.1 に更新 |

## 実装詳細

### ヘルプテキスト設計

**グローバルヘルプ（`-h` / `--help`）:**
```
linear-algebra 1.0.1
Linear algebra vector operations

USAGE:
    linear-algebra <COMMAND> [OPTIONS]

COMMANDS:
    add        Add two vectors
    sub        Subtract two vectors
    scale      Multiply a vector by a scalar
    dot        Compute dot product of two vectors
    sumsq      Compute sum of squares of a vector
    magnitude  Compute magnitude of a vector

OPTIONS:
    -h, --help  Print help information
```

**サブコマンド別ヘルプ（例: `add --help`）:**
```
linear-algebra-add
Add two vectors

USAGE:
    linear-algebra add <vector1> <vector2>

ARGS:
    <vector1>  First vector (comma-separated, e.g. 1,2,3)
    <vector2>  Second vector (comma-separated, e.g. 4,5,6)

OPTIONS:
    -h, --help  Print help information
```

### 引数パースの変更

```rust
// グローバルヘルプ: args[1] が "-h" or "--help"
// サブコマンド別ヘルプ: args に "-h" or "--help" が含まれる場合
fn is_help(args: &[String]) -> bool {
    args.iter().any(|a| a == "-h" || a == "--help")
}
```

### 処理フロー

```
args[1] が "-h" / "--help"
  → print_global_help() → exit(0)

args[1] がサブコマンド && args に "-h" / "--help" が含まれる
  → print_command_help(cmd) → exit(0)

それ以外
  → 既存の演算処理
```

## 影響範囲の分析

- `src/vector.rs` は変更なし（全テスト継続通過）
- `src/main.rs` の `run()` 関数冒頭にヘルプ検出を追加
