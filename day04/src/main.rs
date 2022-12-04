fn get_ranges() -> Vec<(u32, u32, u32, u32)> {
    return include_str!("../input.txt")
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
        .collect::<Vec<(_, _, _, _)>>();
}

fn part1() -> u32 {
    return get_ranges()
        .iter()
        .filter(|values| {
            let first_in_second = (values.0 >= values.2) && (values.1 <= values.3);
            let second_in_first = (values.0 <= values.2) && (values.1 >= values.3);
            first_in_second || second_in_first
        })
        .count() as u32;
}

fn part2() -> u32 {
    return get_ranges()
        .iter()
        .filter(|values| {
            let first_starts_in_second = (values.0 >= values.2) && (values.1 <= values.3);
            let first_ends_in_second = (values.1 >= values.2) && (values.1 <= values.3);
            let second_starts_in_first = (values.3 >= values.0) && (values.2 <= values.1);
            let second_ends_in_first = (values.3 >= values.0) && (values.3 <= values.1);

            first_starts_in_second
                || first_ends_in_second
                || second_starts_in_first
                || second_ends_in_first
        })
        .count() as u32;
}

fn main() {
    println!("day4 part1: {}", part1());
    println!("day4 part2: {}", part2());
}
