use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct FriendshipData {
    pub pairs: Vec<Pair>,
}

#[derive(Debug, Deserialize)]
pub struct Pair {
    pub pair: [u32; 2],
}

pub type FriendMap = HashMap<u32, Vec<u32>>;
