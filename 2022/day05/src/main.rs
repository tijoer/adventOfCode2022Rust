struct StackOfCrates {
    crates: Vec<Vec<char>>,
}

impl StackOfCrates {
    fn get_column(&self, col: i32) -> Vec<char> {
        return self.crates.iter().fold(Vec::<char>::new(), |mut acc, nex| {
            acc.push(*nex.get(col as usize).unwrap());
            return acc;
        });
    }

    fn remove_top_entry(&self, col: i32) -> Vec<char> {

    }
}

pub struct Registers {
    cir: String, // current instruction
    r0: String,
    r1: String,
    r2: String,
    r3: String,
    r4: String,
}

fn parse_stack_of_crates(input: &str) -> StackOfCrates {
    let mut stack_of_crates = StackOfCrates { crates: Vec::new() };

    for line in input.lines().collect::<Vec<&str>>()[0..8].iter() {
        let mut new_line: Vec<char> = Vec::new();

        line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|chunk| chunk.get(1).unwrap())
            .for_each(|_c| new_line.push(*_c));

        stack_of_crates.crates.push(new_line);
    }

    return stack_of_crates;
}

fn move_(registers: Registers) {
    println!("move {} {}", registers.cir, registers.r0);
}

pub fn part1(input: &str) -> i32 {
    let mut stack_of_crates = parse_stack_of_crates(input);
    // dbg!(stack_of_crates.get_column(2));

    for line in input.lines().collect::<Vec<&str>>()[10..].iter() {
        let foo = line.split(' ').collect::<Vec<&str>>()[0];
        let mut registers = Registers {
            cir: line.split(' ').collect::<Vec<&str>>()[0].to_owned(),
            r0: line.split(' ').collect::<Vec<&str>>()[1].to_owned(),
            r1: line.split(' ').collect::<Vec<&str>>()[2].to_owned(),
            r2: line.split(' ').collect::<Vec<&str>>()[3].to_owned(),
            r3: line.split(' ').collect::<Vec<&str>>()[4].to_owned(),
            r4: line.split(' ').collect::<Vec<&str>>()[5].to_owned(),  
        };

        match registers.cir.as_str() {
            "move" => move_(registers),
            _ => panic!("unrecognized opcode"),
        }
    }

    return 0;
}

fn main() {
    println!("part1 {}", part1(include_str!("../input.txt")));
}
