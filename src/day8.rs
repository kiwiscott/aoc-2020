use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Instructions {
    parse_s_input(input).unwrap()
}

#[aoc(day8, part1)]
fn part1(instructions: &Instructions) -> i64 {
    let mut machine = Machine::new(instructions.clone());
    match machine.run() {
        Some(MachineState::Compelte) => {
            println!("-Instructions Accumulator result:{:?}", machine.accumulator)
        }
        Some(MachineState::InfiniteLoop) => {
            println!("-Infinite Loop Detected:{:?}", machine.accumulator)
        }
        None => panic!("-Recursion Detected e:{:?}"),
    };
    machine.accumulator
}

#[aoc(day8, part2)]
fn part2(instructions: &Instructions) -> i64 {
    let to_test: Vec<usize> = instructions
        .iter()
        .enumerate()
        .filter(|(_i, x)| match x {
            Instruction::Nop(_) => true,
            Instruction::Jmp(_) => true,
            _ => false,
        })
        .map(|(i, _x)| i)
        .collect();

    let mut r = 0;

    for index in to_test {
        let mut instructions2 = instructions.clone();

        let item = instructions2.get_mut(index).unwrap();
        match item {
            Instruction::Nop(os) => *item = Instruction::Jmp(*os),
            Instruction::Jmp(os) => *item = Instruction::Nop(*os),
            _ => (),
        }
        let mut m = Machine::new(instructions2);
        match m.run() {
            Some(MachineState::Compelte) => {
                println!(
                    "\n-Instructions Accumulator : {:?} Changed : {:?}",
                    m.accumulator, index
                );
                r = m.accumulator
            }
            Some(MachineState::InfiniteLoop) => (),
            None => panic!("."),
        };
    }
    r
}

pub type Instructions = Vec<Instruction>;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
    End,
}

#[derive(Debug, Copy, Clone)]
pub enum MachineState {
    Compelte,
    InfiniteLoop,
}

pub struct Machine {
    pub accumulator: i64,
    pub instructions: Instructions,
}

impl Machine {
    pub fn new(instructions: Instructions) -> Self {
        Machine {
            accumulator: 0,
            instructions: instructions,
        }
    }

    pub fn run(&mut self) -> Option<MachineState> {
        let mut next_index: i64 = 0;
        let mut processed = HashSet::<i64>::new();

        loop {
            if !processed.insert(next_index) {
                return Some(MachineState::InfiniteLoop);
            }

            match self.instructions.get(next_index as usize) {
                Some(Instruction::Nop(_)) => next_index += 1,
                Some(Instruction::Acc(offset)) => {
                    next_index += 1;
                    self.accumulator += offset;
                }
                Some(Instruction::Jmp(offset)) => {
                    next_index += offset;
                }
                Some(Instruction::End) => break,
                None => (),
            };
        }

        return Some(MachineState::Compelte);
    }
}

pub fn parse_s_input(input: &str) -> Option<Instructions> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"^(\w{3}).*([\+|-]\d*)").unwrap();
    }

    let mut ps = Instructions::new();
    for line in input.lines() {
        let caps = RE.captures(line).unwrap();
        let os = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let op = match caps.get(1).unwrap().as_str() {
            "nop" => Instruction::Nop(os),
            "acc" => Instruction::Acc(os),
            "jmp" => Instruction::Jmp(os),
            _ => panic!("Type Not Expected"),
        };
        ps.push(op);
    }
    ps.push(Instruction::End);
    Some(ps)
}
