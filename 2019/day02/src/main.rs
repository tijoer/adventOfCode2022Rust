use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive, Debug, PartialEq, Eq)]
pub enum Opcodes {
    ADD = 1,
    MULT = 2,
    HALT = 99,
}

#[derive(FromPrimitive, Debug, PartialEq, Eq)]
pub enum States {
    RUNNING = 0,
    HALTED = 1,
    ERROR = 99,
}

pub struct Registers {
    pc: i32,      // programm counter
    cir: Opcodes, // current instruction
    cpu_state: States,
    r0: i32,
    r1: i32,
    r2: i32,
    r3: i32,
}

pub fn part1(input: &str) -> i32 {
    let heap = input.split(',').collect::<Vec<&str>>();
    let mut registers = Registers {
        pc: 0,
        cir: Opcodes::ADD,
        cpu_state: States::RUNNING,
        r0: 0,
        r1: 0,
        r2: 0,
        r3: 0,
    };

    while registers.cpu_state == States::RUNNING {
        let current_instruction: Opcodes = FromPrimitive::from_i32(
            heap.get(registers.pc as usize)
                .unwrap()
                .parse::<i32>()
                .unwrap(),
        )
        .unwrap();

        match current_instruction {
            Opcodes::ADD => add(),
            Opcodes::MULT => println!("TODO unimplemented mult"),
            Opcodes::HALT => println!("TODO unimplemented halt"),

            _ => {
                println!("ERROR");
                registers.cpu_state = States::ERROR;
            }
        }

        registers.cpu_state = States::HALTED;
    }

    return 0;
}

fn add() {
    println!("ADD");
}

fn main() {
    println!("part1 {}", part1(include_str!("../input.txt")));
}
