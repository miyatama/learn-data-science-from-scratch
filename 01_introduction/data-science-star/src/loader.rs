use crate::models::{FriendshipData, User};
use std::fs;

pub fn load_users(path: &str) -> Vec<User> {
    let content = fs::read_to_string(path).expect("users.jsonの読み込みに失敗しました");
    serde_json::from_str(&content).expect("users.jsonのパースに失敗しました")
}

pub fn load_friendships(path: &str) -> FriendshipData {
    let content = fs::read_to_string(path).expect("friendship.jsonの読み込みに失敗しました");
    serde_json::from_str(&content).expect("friendship.jsonのパースに失敗しました")
}
