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

--- Part Two ---

As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.

Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.

Again considering the example above, the crates begin in the same configuration:

    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

Moving a single crate from stack 2 to stack 1 behaves the same as before:

[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3 

However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:

        [D]
        [N]
    [C] [Z]
    [M] [P]
 1   2   3

Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:

        [D]
        [N]
[C]     [Z]
[M]     [P]
 1   2   3

Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:

        [D]
        [N]
        [Z]
[M] [C] [P]
 1   2   3

In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?


*/

use core::fmt;
use regex::Regex;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::num::ParseIntError;

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
    n: usize,
    from: usize,
    to: usize,
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
    // Reverse so column #'s are first
    let crate_re = Regex::new(r"(\[[A-Z]\])(?:\s|$)").unwrap();
    for line in stack_string.lines().rev() {
        if line.contains("0123456789") {
            continue;
        }
        for cr in crate_re.find_iter(line) {
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
    let instr_string = two_halves[1];
    let mut instructions = Vec::new();
    let instr_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for line in instr_string.lines() {
        let m = instr_re
            .captures(line)
            .expect("Instruction line won't parse");
        let n: usize = m.get(1).unwrap().as_str().parse().unwrap();
        let from: usize = m.get(2).unwrap().as_str().parse().unwrap();
        let to: usize = m.get(3).unwrap().as_str().parse().unwrap();
        instructions.push(Instruction { n, from, to });
    }
    Ok((stacks, instructions))
}

fn apply_instructions(stacks: &mut Vec<Stack>, instructions: &Vec<Instruction>) {
    for instruction in instructions.iter() {
        for _i in 0..instruction.n {
            let this_crate: Crate = stacks[instruction.from as usize - 1]
                .pop()
                .expect("Not enough items to move!");
            stacks[instruction.to as usize - 1].push(this_crate);
        }
    }
}

fn apply_instructions_9001(stacks: &mut Vec<Stack>, instructions: &Vec<Instruction>) {
    for instruction in instructions.iter() {
        let from_stack = &mut stacks[instruction.from - 1];
        let to_move = from_stack.len() - instruction.n;
        let mut moved_crates: Vec<Crate> = from_stack.split_off(to_move);
        let to_stack = &mut stacks[instruction.to - 1];
        to_stack.append(&mut moved_crates);
    }
}

#[test]
fn test_instructions() {
    let mut test_stacks: Vec<Stack> = Vec::new();
    test_stacks.push(vec!['Z', 'N']);
    test_stacks.push(vec!['M', 'C', 'D']);
    test_stacks.push(vec!['P']);
    let test_instructions: Vec<Instruction> = vec![
        Instruction {
            n: 1,
            from: 2,
            to: 1,
        },
        Instruction {
            n: 3,
            from: 1,
            to: 3,
        },
        Instruction {
            n: 2,
            from: 2,
            to: 1,
        },
        Instruction {
            n: 1,
            from: 1,
            to: 2,
        },
    ];
    let test_stacks = test_stacks;
    let (mut parsed_stacks, instructions) = parse_drawing(TEST_INPUT).unwrap();
    assert_eq!(test_stacks, parsed_stacks);
    assert_eq!(test_instructions, instructions);
    let mut moved_test_stacks: Vec<Stack> = Vec::new();
    moved_test_stacks.push(vec!['C']);
    moved_test_stacks.push(vec!['M']);
    moved_test_stacks.push(vec!['P', 'D', 'N', 'Z']);
    let mut save_parsed_stacks = parsed_stacks.clone();
    apply_instructions(&mut parsed_stacks, &instructions);
    assert_eq!(moved_test_stacks, parsed_stacks);
    let mut moved_9001_stacks: Vec<Stack> = Vec::new();
    moved_9001_stacks.push(vec!['M']);
    moved_9001_stacks.push(vec!['C']);
    moved_9001_stacks.push(vec!['P', 'Z', 'N', 'D']);
    apply_instructions_9001(&mut save_parsed_stacks, &instructions);
    assert_eq!(moved_9001_stacks, save_parsed_stacks);
}

fn top_stacks(stacks: &mut Vec<Stack>) -> String {
    stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect::<String>()
}

fn main() {
    let buf = fs::read_to_string("2022d5p1.txt").unwrap();
    let (mut parsed_stacks, instructions) = parse_drawing(&buf).unwrap();
    let mut saved_parsed_stacks = parsed_stacks.clone();
    apply_instructions(&mut parsed_stacks, &instructions);
    println!("The answer is: {}", top_stacks(&mut parsed_stacks));
    apply_instructions_9001(&mut saved_parsed_stacks, &instructions);
    println!("9001 Answer is: {}", top_stacks(&mut saved_parsed_stacks));
}
