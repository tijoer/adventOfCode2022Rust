
pub fn solve() -> (u32, u32) {
    let mut part_1_score: u32 = 0;
    let mut part_2_score: u32 = 0;

    let foo = include_str!("../input.txt").lines().map(|x| {
        dbg!(x);
    });

    // dbg!(foo);
    return (part_1_score, part_2_score);
}

fn main() {
    let result = solve();
    assert_eq!(result.0, 0);
    assert_eq!(result.1, 0);
    println!("day4 part1: {}", result.0);
    println!("day4 part2: {}", result.1);
}

