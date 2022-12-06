#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

struct StackOfCrates {
    pub crates: Vec<Vec<char>>,
}

impl StackOfCrates {
    fn get_column_crates(&self, col: i32) -> Vec<char> {
        return self.crates.iter().fold(Vec::<char>::new(), |mut acc, nex| {
            acc.push(*nex.get(col as usize).unwrap());
            return acc;
        });
    }

    fn remove_top_entry(&mut self, col: i32) -> char {
        for row in self.crates.iter_mut() {
            let mut crate_ = row.get_mut(col as usize).unwrap();
            if ' '.ne(crate_) {
                let value = *crate_;
                *crate_ = ' ';
                return value;
            }
        }
        panic!("Tried to remove from empty column.");
    }

    fn add_crate_to_column(&self, col: i32, crate_: char) {
        if ' '.ne(self.crates.get(0).unwrap().get(col as usize).unwrap()) { //add new row at top
            
        }
    }

    fn parse_stack_of_crates(input: &str) ->  Vec<Vec<char>> {
        let mut stack_of_crates = Vec::new();
    
        for line in input.lines().collect::<Vec<&str>>()[0..8].iter() {
            let mut new_line: Vec<char> = Vec::new();
    
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|chunk| chunk.get(1).unwrap())
                .for_each(|c| new_line.push(*c));
    
            stack_of_crates.push(new_line);
        }
    
        return stack_of_crates;
    }

    pub fn new(input: &str) -> Self {
        StackOfCrates {
            crates: StackOfCrates::parse_stack_of_crates(input)
        }
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

fn move_(registers: Registers, stack_of_crates:  &mut StackOfCrates) {
    let mut amount: i32 = registers.r0.parse().unwrap();
    let mut from: i32 = registers.r2.parse().unwrap();
    let mut to: i32 = registers.r4.parse().unwrap();

    from -= 1;
    to -= 1;

    for i in 0..amount {
        let crate_ = stack_of_crates.remove_top_entry(from);
        stack_of_crates.add_crate_to_column(crate_);
    }
}

pub fn part1(input: &str) -> i32 {
    let mut stack_of_crates = StackOfCrates::new(input);
    let mut soc_reference: StackOfCrates = stack_of_crates;

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
            // Match opcodes. Currently just one code.
            "move" => move_(registers, &mut soc_reference),
            _ => panic!("unrecognized opcode"),
        }
    }

    return 0;
}

fn main() {
    println!("part1 {}", part1(include_str!("../input.txt")));
}
