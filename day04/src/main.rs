pub fn solve() -> (u32, u32) {
    let mut _part_2_score: u32 = 0;

    let _part_1_score = include_str!("../input.txt")
        .lines()
        .map(|lines| {
            let (elf_0, elf_1) = lines.split_once(',').unwrap();
            let (range_0_start, range_0_end) = elf_0
                .split_once('-')
                .map(|x| (x.0.parse::<u32>().unwrap(), x.1.parse::<u32>().unwrap()))
                .unwrap();

            let (range_1_start, range_1_end) = elf_1
                .split_once('-')
                .map(|x| (x.0.parse::<u32>().unwrap(), x.1.parse::<u32>().unwrap()))
                .unwrap();

            (range_0_start, range_0_end, range_1_start, range_1_end)
        })
        .filter(|_range_values| {
            let first_in_second =
                (_range_values.0 >= _range_values.2) && (_range_values.1 <= _range_values.3);
            let second_in_first =
                (_range_values.0 <= _range_values.2) && (_range_values.1 >= _range_values.3);
            first_in_second || second_in_first
        })
        .count();

    return (_part_1_score as u32, _part_2_score as u32);
}

fn main() {
    let result = solve();
    assert_eq!(result.0, 602);
    // assert_eq!(result.1, 0);
    println!("day4 part1: {}", result.0);
    println!("day4 part2: {}", result.1);
}
