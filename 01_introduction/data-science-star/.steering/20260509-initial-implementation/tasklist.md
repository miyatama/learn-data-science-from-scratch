# 初回実装 タスクリスト

## タスク一覧

| # | タスク | 状態 |
|---|--------|------|
| 1 | Cargo.toml に serde / serde_json を追加 | [x] |
| 2 | `src/models.rs` を作成（User, FriendshipData, Pair, FriendMap） | [x] |
| 3 | `src/loader.rs` を作成（load_users, load_friendships） | [x] |
| 4 | `src/network.rs` を作成（build_friend_map, friends_of_friends） | [x] |
| 5 | `src/stats.rs` を作成（number_of_friends, average_number_of_friends, common_friends, degree_centrality） | [x] |
| 6 | `src/main.rs` を書き換え（全機能の呼び出しと出力） | [x] |
| 7 | `cargo build` でビルド確認 | [x] |
| 8 | `cargo clippy` で警告なしを確認 | [x] |
| 9 | `cargo fmt` を適用 | [x] |
| 10 | `cargo test` で全テストパスを確認 | [x] |
| 11 | `cargo run` で出力結果を手動検証 | [x] |

## 完了条件

- `cargo run` が正常終了し、全統計量が出力される
- `cargo clippy` 警告ゼロ
- `cargo test` 全パス
