/*
--- Day 5: Supply Stacks ---

The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3 

In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3

Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3

Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3

The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?

*/

use core::fmt;
use std::error::Error;
use std::fmt::{Debug,Formatter};
use std::fs;
use std::num::ParseIntError;
use regex::Regex;

const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

type Crate = char;
type Stack = Vec<Crate>;

#[derive(PartialEq)]
struct Instruction {
    n: i32,
    from: i32,
    to: i32,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "move {} from {} to {}", self.n, self.from, self.to)
    }
}


fn parse_drawing(input: &str) -> Result<(Vec<Stack>, Vec<Instruction>), ParseIntError> {
    let mut stacks: Vec<Stack> = Vec::new();
    let two_halves: Vec<&str> = input.split("\n\n").collect();
    let stack_string = two_halves[0];
    let instr_string = two_halves[1];
    // Reverse so column #'s are first
    let crate_re = Regex::new(r"(\[[A-Z]\])(?:\s|$)").unwrap();
    for line in stack_string.lines().rev() {
        if line.contains("0123456789") {
            continue;
        }
        for cr in crate_re.find_iter(line) {
            println!("{:?} - {:?}", cr, cr.as_str());
            let col = cr.start() / 4;
            let crate_char = cr.as_str().chars().nth(1).expect("Should have a char here");
            loop {
                if stacks.len() <= col {
                    stacks.push(Vec::new());
                } else {
                    break;
                }
            }
            stacks[col].push(crate_char);
        }
    }
    let mut instructions = Vec::new();
    Ok((stacks, instructions))
}

#[test]
fn test_instructions() {
    let mut test_stacks: Vec<Stack> = Vec::new();
    test_stacks.push(vec!['Z', 'N']);
    test_stacks.push(vec!['M', 'C', 'D']);
    test_stacks.push(vec!['P']);
    let test_instructions: Vec<Instruction> = vec![
        Instruction { n: 1, from: 2, to: 1},
        Instruction { n: 3, from: 1, to: 3},
        Instruction { n: 2, from: 2, to: 1},
        Instruction { n: 1, from: 1, to: 2},
    ];
    let test_stacks = test_stacks;
    let (parsed_stacks, instructions) = parse_drawing(TEST_INPUT).unwrap();
    assert_eq!(test_stacks, parsed_stacks);
    assert_eq!(test_instructions, instructions);
}

fn main() {
    let buf = fs::read_to_string("2022d4p1.txt").unwrap();
}
