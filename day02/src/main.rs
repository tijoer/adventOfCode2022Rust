use std::collections::HashMap;

fn solve() -> (u32, u32) {
    let mut result_hash_map: HashMap<_, _> = HashMap::new();

    result_hash_map.insert("A X", (4, 3));
    result_hash_map.insert("A Y", (8, 4));
    result_hash_map.insert("A Z", (3, 8));
    result_hash_map.insert("B X", (1, 1));
    result_hash_map.insert("B Y", (5, 5));
    result_hash_map.insert("B Z", (9, 9));
    result_hash_map.insert("C X", (7, 2));
    result_hash_map.insert("C Y", (2, 6));
    result_hash_map.insert("C Z", (6, 7));

    return include_str!("../input.txt")
    .lines()
    .fold((0, 0), |cur, nxt| {
        let foo = result_hash_map.get(nxt).unwrap().0;
        let bar = result_hash_map.get(nxt).unwrap().1;
        (cur.0 + foo, cur.1 + bar)
    });
}

fn main() {
    println!("part1: {}", solve().0);
    println!("part1: {}", solve().1);
}
