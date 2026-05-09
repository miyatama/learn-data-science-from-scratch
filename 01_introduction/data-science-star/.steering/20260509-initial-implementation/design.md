# 初回実装 設計

## 実装アプローチ

serde_jsonでJSONを読み込み、モジュールに分割して各統計量を算出する。
エラーハンドリングは `Result` / `expect()` で行い、`unwrap()` は使用しない。

## 変更するコンポーネント

### Cargo.toml

serde / serde_json を追加する。

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### ファイル構成

```
src/
├── main.rs       # 既存ファイルを全面書き換え
├── models.rs     # 新規作成
├── loader.rs     # 新規作成
├── network.rs    # 新規作成
└── stats.rs      # 新規作成
```

## データ構造

```rust
// models.rs
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

// friendship.json のパース用
#[derive(Debug, Deserialize)]
pub struct FriendshipData {
    pub pairs: Vec<Pair>,
}

#[derive(Debug, Deserialize)]
pub struct Pair {
    pub pair: [u32; 2],
}

// 隣接リスト
pub type FriendMap = HashMap<u32, Vec<u32>>;
```

## 各モジュールの実装詳細

### loader.rs

```rust
pub fn load_users(path: &str) -> Vec<User>
// fs::read_to_string → serde_json::from_str

pub fn load_friendships(path: &str) -> FriendshipData
// fs::read_to_string → serde_json::from_str
```

### network.rs

```rust
pub fn build_friend_map(users: &[User], pairs: &[Pair]) -> FriendMap
// 各ペアを双方向に登録する

pub fn friends_of_friends(user_id: u32, friend_map: &FriendMap) -> Vec<u32>
// 友人の友人を収集し、自身と直接友人を除いて重複排除して返す
```

### stats.rs

```rust
pub fn number_of_friends(user_id: u32, friend_map: &FriendMap) -> usize
// friend_map[user_id].len()

pub fn average_number_of_friends(friend_map: &FriendMap) -> f64
// 全友人数の合計 / ユーザー数

pub fn common_friends(user_id_a: u32, user_id_b: u32, friend_map: &FriendMap) -> usize
// AとBの友人リストの積集合のサイズ

pub fn degree_centrality(user_id: u32, friend_map: &FriendMap, total_users: usize) -> f64
// number_of_friends(user_id) as f64 / (total_users - 1) as f64
```

### main.rs

```rust
fn main() {
    // 1. データ読み込み
    // 2. 隣接リスト構築
    // 3. 友人数の表示
    // 4. 平均友人数の表示
    // 5. 友人の友人の表示
    // 6. 共通友人数の表示（全ペア）
    // 7. 次数中心性の表示
}
```

## 影響範囲の分析

- 既存の `src/main.rs` を全面書き換え（現在は `Hello, world!` のみ）
- `data/` ディレクトリは変更なし
- `Cargo.toml` に依存クレートを追加
