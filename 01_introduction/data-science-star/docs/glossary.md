# ユビキタス言語定義

## ドメイン用語

| 日本語 | 英語 | 定義 |
|--------|------|------|
| ユーザー | User | データサイエンススター社の社員。IDと名前を持つ |
| 交友関係 | Friendship | 2人のユーザー間の双方向のつながり |
| 友人 | Friend | 交友関係で直接つながっているユーザー |
| 友人の友人 | Friend of Friend | 直接の友人を介してつながるユーザー（自身・直接友人を除く） |
| 隣接リスト | Friend Map | ユーザーIDをキー、友人IDリストを値とするマップ |
| 共通友人 | Common Friend | 2人のユーザーが共通して持つ友人 |

## ネットワーク分析用語

| 日本語 | 英語 | 定義 |
|--------|------|------|
| 次数 | Degree | あるノード（ユーザー）が持つエッジ（交友関係）の数。友人数と同義 |
| 次数中心性 | Degree Centrality | 次数を最大可能次数（全ユーザー数 - 1）で割った値。0〜1の範囲をとる |
| 平均友人数 | Average Number of Friends | 全ユーザーの友人数の算術平均 |

## コード上の命名規則

| 概念 | コード上の名前 | 型 |
|------|--------------|-----|
| ユーザー | `User` | struct |
| 隣接リスト | `FriendMap` | `HashMap<u32, Vec<u32>>` |
| 友人数 | `number_of_friends` | `usize` |
| 平均友人数 | `average_number_of_friends` | `f64` |
| 共通友人数 | `common_friends` | `usize` |
| 次数中心性 | `degree_centrality` | `f64` |
| 友人の友人 | `friends_of_friends` | `Vec<u32>` |
