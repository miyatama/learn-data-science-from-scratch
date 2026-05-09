use crate::models::{FriendMap, Pair, User};
use std::collections::HashMap;

pub fn build_friend_map(users: &[User], pairs: &[Pair]) -> FriendMap {
    let mut map: FriendMap = HashMap::new();
    for user in users {
        map.insert(user.id, Vec::new());
    }
    for pair in pairs {
        let [a, b] = pair.pair;
        map.entry(a).or_default().push(b);
        map.entry(b).or_default().push(a);
    }
    map
}

pub fn friends_of_friends(user_id: u32, friend_map: &FriendMap) -> Vec<u32> {
    let direct: std::collections::HashSet<u32> = friend_map
        .get(&user_id)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .collect();

    let mut result: std::collections::HashSet<u32> = std::collections::HashSet::new();
    for &friend_id in &direct {
        if let Some(fof_list) = friend_map.get(&friend_id) {
            for &fof in fof_list {
                if fof != user_id && !direct.contains(&fof) {
                    result.insert(fof);
                }
            }
        }
    }
    let mut result: Vec<u32> = result.into_iter().collect();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Pair;

    fn make_users(ids: &[u32]) -> Vec<crate::models::User> {
        ids.iter()
            .map(|&id| crate::models::User {
                id,
                name: format!("User{}", id),
            })
            .collect()
    }

    fn make_pairs(pairs: &[[u32; 2]]) -> Vec<Pair> {
        pairs.iter().map(|&pair| Pair { pair }).collect()
    }

    #[test]
    fn test_build_friend_map() {
        let users = make_users(&[0, 1, 2]);
        let pairs = make_pairs(&[[0, 1], [1, 2]]);
        let map = build_friend_map(&users, &pairs);
        assert!(map[&0].contains(&1));
        assert!(map[&1].contains(&0));
        assert!(map[&1].contains(&2));
        assert!(map[&2].contains(&1));
    }

    #[test]
    fn test_friends_of_friends() {
        let users = make_users(&[0, 1, 2, 3]);
        let pairs = make_pairs(&[[0, 1], [1, 2], [1, 3]]);
        let map = build_friend_map(&users, &pairs);
        let fof = friends_of_friends(0, &map);
        assert!(fof.contains(&2));
        assert!(fof.contains(&3));
        assert!(!fof.contains(&0));
        assert!(!fof.contains(&1));
    }
}
