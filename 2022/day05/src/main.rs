struct Stack_of_crates {
    crates: Vec<Vec<char>>
}

fn parse_stack_of_crates(input: &str) -> Stack_of_crates {
    let mut stack_of_crates = Stack_of_crates {
        crates: Vec::new()
    }

    for line in input.lines().collect::<Vec<&str>>()[0..8].iter() {
        let mut new_line: Vec<char> = Vec::new();

        line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|chunk| chunk.get(1).unwrap())
            .for_each(|_c| { new_line.push(*_c)});
        
            stack_of_crates.crates.push(new_line);
    }

    return stack_of_crates;
}

fn get_stack_of_crates_column(col: i32) -> Vec<char> {

    return Vec::new();
}

pub fn part1(input: &str) -> i32 {
    // dbg!(parse_stack_of_crates(input).crates.get_stack_of_crates_column(0));

    return 0;
}

fn part2() -> i32 {
    return 0;
}

fn main() {
    println!("part1 {}", part1(include_str!("../input.txt")));
    println!("day05 part2: {}", part2());
}
