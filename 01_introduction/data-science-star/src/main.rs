mod loader;
mod models;
mod network;
mod stats;

use network::build_friend_map;
use stats::{average_number_of_friends, common_friends, degree_centrality, number_of_friends};

fn main() {
    let users = loader::load_users("data/users.json");
    let friendship_data = loader::load_friendships("data/friendship.json");
    let friend_map = build_friend_map(&users, &friendship_data.pairs);
    let total_users = users.len();

    // 友人数
    println!("=== 友人数 ===");
    for user in &users {
        let n = number_of_friends(user.id, &friend_map);
        println!("{}: {}人", user.name, n);
    }

    // 平均友人数
    println!("\n=== 平均友人数 ===");
    println!("平均友人数: {:.2}", average_number_of_friends(&friend_map));

    // 友人の友人
    println!("\n=== 友人の友人 ===");
    for user in &users {
        let fof_ids = network::friends_of_friends(user.id, &friend_map);
        let fof_names: Vec<&str> = fof_ids
            .iter()
            .filter_map(|id| users.iter().find(|u| u.id == *id))
            .map(|u| u.name.as_str())
            .collect();
        println!("{} の友人の友人: {:?}", user.name, fof_names);
    }

    // 共通友人数（全ペア）
    println!("\n=== 共通友人数 ===");
    for i in 0..users.len() {
        for j in (i + 1)..users.len() {
            let n = common_friends(users[i].id, users[j].id, &friend_map);
            if n > 0 {
                println!("{} & {}: {}人の共通友人", users[i].name, users[j].name, n);
            }
        }
    }

    // 次数中心性
    println!("\n=== 次数中心性 ===");
    for user in &users {
        let dc = degree_centrality(user.id, &friend_map, total_users);
        println!("{}: {:.3}", user.name, dc);
    }
}
