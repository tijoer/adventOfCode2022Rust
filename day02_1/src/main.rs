use std::collections::HashMap;

fn main() {
    let mut result_hash_map: HashMap<_, _> = HashMap::new();

    result_hash_map.insert("A X", 4);
    result_hash_map.insert("A Y", 8);
    result_hash_map.insert("A Z", 3);
    result_hash_map.insert("B X", 1);
    result_hash_map.insert("B Y", 5);
    result_hash_map.insert("B Z", 9);
    result_hash_map.insert("C X", 7);
    result_hash_map.insert("C Y", 2);
    result_hash_map.insert("C Z", 6);

    let result: u32 = include_str!("../input.txt")
        .lines()
        .map(|line_score| {
            return result_hash_map.get(line_score).unwrap();
        })
        .collect::<Vec<_>>()
        .into_iter()
        .sum();

    println!("{}", result);
}
