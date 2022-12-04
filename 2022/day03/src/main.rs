
pub fn solve() -> (u32, u32) {
    let a_z = ('a'..='z').chain('A'..='Z').into_iter().collect::<Vec<char>>();
    let mut final_score: u32 = 0;
    let mut part_2_score: u32 = 0;
    let mut elf_group: Vec::<String> = Vec::new();

    for line in include_str!("../input.txt").lines() {
        let first_compartment_end_position = line.len() / 2;
        let first_compartment = line.split_at(first_compartment_end_position).0;
        let second_compartment = line.split_at(first_compartment_end_position).1;

        let found_char = first_compartment.chars().find(|c| second_compartment.contains(*c)).unwrap();
        let elf_score = a_z.iter().position(|&x| x == found_char).unwrap();
        final_score += elf_score as u32;
        final_score += 1; // Fuck this, lol :D. Might read the assigment correctly next time.

        elf_group.push(line.to_string());

        let mut match_in_all_three: char;
        let mut badge_char: char = '0';
        if elf_group.len() == 3 {
            let elf_0 = elf_group.get(0);
            let elf_1 = elf_group.get(1);
            let elf_2 = elf_group.get(2);

            for c in elf_0.unwrap().chars() {
                let foo = elf_1.unwrap().chars().find(|x| *x == c);

                if foo != None {
                    println!("xxx {}", foo.unwrap());
                    let bar = elf_2.unwrap().chars().find(|x| *x == foo.unwrap());
                    if bar != None {
                        println!("xxx MATCH IN ALL 3 {}", bar.unwrap());
                        badge_char = bar.unwrap();
                    }
                } else {
                    println!("xxx ");
                }
            }
            println!("------");

            let badge_score = a_z.iter().position(|&x| x == badge_char).unwrap();
            part_2_score += badge_score as u32;
            part_2_score += 1;

            elf_group.clear();
        }
    }

    return (final_score, part_2_score);
}

fn main() {
    let result = solve();
    assert_eq!(result.0, 7691);
    assert_eq!(result.1, 2508);
    println!("part1: {}", result.0);
    println!("part2: {}", result.1);
}

