use std::collections::HashMap;

fn calculate_result(result_hash_map: HashMap<&str, (u32, u32)>) -> (u32, u32) {
    let mut sums = (0, 0);

    include_str!("../input.txt")
        .lines()
        .map(|line_score| {
            sums.0 += result_hash_map.get(line_score).unwrap().0;
            sums.1 += result_hash_map.get(line_score).unwrap().1;
        })
        .collect::<Vec<_>>();

    return sums;
}

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

    return calculate_result(result_hash_map);
}

fn main() {
    println!("part1: {}", solve().0);
    println!("part1: {}", solve().1);
}
