use crate::models::FriendMap;
use std::collections::HashSet;

pub fn number_of_friends(user_id: u32, friend_map: &FriendMap) -> usize {
    friend_map.get(&user_id).map_or(0, |v| v.len())
}

pub fn average_number_of_friends(friend_map: &FriendMap) -> f64 {
    if friend_map.is_empty() {
        return 0.0;
    }
    let total: usize = friend_map.values().map(|v| v.len()).sum();
    total as f64 / friend_map.len() as f64
}

pub fn common_friends(user_id_a: u32, user_id_b: u32, friend_map: &FriendMap) -> usize {
    let a: HashSet<u32> = friend_map
        .get(&user_id_a)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .collect();
    let b: HashSet<u32> = friend_map
        .get(&user_id_b)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .collect();
    a.intersection(&b).count()
}

pub fn degree_centrality(user_id: u32, friend_map: &FriendMap, total_users: usize) -> f64 {
    if total_users <= 1 {
        return 0.0;
    }
    number_of_friends(user_id, friend_map) as f64 / (total_users - 1) as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_map(pairs: &[(u32, Vec<u32>)]) -> FriendMap {
        pairs.iter().cloned().collect::<HashMap<_, _>>()
    }

    #[test]
    fn test_number_of_friends() {
        let map = make_map(&[(0, vec![1, 2]), (1, vec![0]), (2, vec![0])]);
        assert_eq!(number_of_friends(0, &map), 2);
        assert_eq!(number_of_friends(1, &map), 1);
    }

    #[test]
    fn test_average_number_of_friends() {
        let map = make_map(&[(0, vec![1, 2]), (1, vec![0]), (2, vec![0])]);
        let avg = average_number_of_friends(&map);
        assert!((avg - 4.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_common_friends() {
        let map = make_map(&[
            (0, vec![1, 2]),
            (1, vec![0, 2]),
            (2, vec![0, 1]),
            (3, vec![1]),
        ]);
        assert_eq!(common_friends(0, 3, &map), 1);
        assert_eq!(common_friends(0, 1, &map), 1);
    }

    #[test]
    fn test_degree_centrality() {
        let map = make_map(&[(0, vec![1, 2]), (1, vec![0]), (2, vec![0])]);
        let dc = degree_centrality(0, &map, 3);
        assert!((dc - 1.0).abs() < 1e-9);
        let dc1 = degree_centrality(1, &map, 3);
        assert!((dc1 - 0.5).abs() < 1e-9);
    }
}
