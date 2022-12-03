
pub fn part1() -> (u32, u32) {
    let a_Z = ('a'..='z').chain('A'..='Z').into_iter().collect::<Vec<char>>();
    let mut final_score: u32 = 0;

    for line in include_str!("../input.txt").lines() {
        let first_compartment_end_position = line.len() / 2;
        let first_compartment = line.split_at(first_compartment_end_position).0;
        let second_compartment = line.split_at(first_compartment_end_position).1;

        let found_char = first_compartment.chars().find(|c| second_compartment.contains(*c)).unwrap();
        let elf_score = a_Z.iter().position(|&x| x == found_char).unwrap();
        final_score += elf_score as u32;
        final_score += 1;
        // println!("foo {} {} {} {} {} {}", first_compartment_end_position, line, first_compartment, second_compartment, found_char, elf_score);
    }

    return (final_score, 0);
}

fn main() {
    assert_eq!(part1().0, 7691);
    println!("part2: {}", part1().1);
}

