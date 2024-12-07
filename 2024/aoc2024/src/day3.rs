/*
--- Day 3: Mull It Over ---

"Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're welcome to check the warehouse, though," says the mildly flustered shopkeeper at the North Pole Toboggan Rental Shop. The Historians head out to take a look.

The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"

The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted. All of the instructions have been jumbled up!

It seems like the goal of the program is just to multiply some numbers. It does that with instructions like mul(X,Y), where X and Y are each 1-3 digit numbers. For instance, mul(44,46) multiplies 44 by 46 to get a result of 2024. Similarly, mul(123,4) would multiply 123 by 4.

However, because the program's memory has been corrupted, there are also many invalid characters that should be ignored, even if they look like part of a mul instruction. Sequences like mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.

For example, consider the following section of corrupted memory:

    xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

Only the four highlighted sections are real mul instructions. Adding up the result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).

Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the results of the multiplications?

--- Part Two ---

As you scan through the corrupted memory, you notice that some of the conditional statements are also still intact. If you handle some of the uncorrupted conditional statements in the program, you might be able to get an even more accurate result.

There are two new instructions you'll need to handle:

    The do() instruction enables future mul instructions.
    The don't() instruction disables future mul instructions.

Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions are enabled.

For example:

xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))

This corrupted memory is similar to the example from before, but this time the mul(5,5) and mul(11,8) instructions are disabled because there is a don't() instruction before them. The other mul instructions function normally, including the one at the end that gets re-enabled by a do() instruction.

This time, the sum of the results is 48 (2*4 + 8*5).

Handle the new instructions; what do you get if you add up all of the results of just the enabled multiplications?


*/

use crate::util::parse_file;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    borrow::Borrow,
    fs::File,
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
};

pub fn day3(path: Option<PathBuf>) {
    let input = match path.clone() {
        None => File::open("day3test.txt"),
        Some(path) => File::open(path),
    };
    let muls = parse_file(input.unwrap(), parser);
    let sum: i32 = muls.iter().sum();
    println!("{:?} sum = {}", muls, sum);
    let input = match path {
        None => File::open("day3p2test.txt"),
        Some(path) => File::open(path),
    };
    let muls2 = parse_file(input.unwrap(), parserp2);
    let sum: i32 = muls2.iter().sum();
    println!("{:?} sum = {}", muls2, sum);
}

fn parser(line: String) -> i32 {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap());
    RE.captures_iter(&line)
        .map(|c| {
            let (_, [sv1, sv2]) = c.extract();
            let v1: i32 = sv1.parse::<_>().unwrap();
            let v2: i32 = sv2.parse::<_>().unwrap();
            let product = v1 * v2;
            println!("{v1} * {v2} = {}", product);
            product
        })
        .sum()
}

static ENABLED: AtomicBool = AtomicBool::new(true);

fn parserp2(line: String) -> i32 {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r#"(don't\(\)|do\(\)|mul\([0-9]{1,3},[0-9]{1,3}\))"#).unwrap());
    static SUBRE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap());
    RE.find_iter(&line)
        .filter_map(|m| {
            let cmd = m.as_str();
            if cmd == "do()" {
                ENABLED.store(true, Ordering::Relaxed);
                None
            } else if cmd == "don't()" {
                ENABLED.store(false, Ordering::Relaxed);
                None
            } else {
                if ENABLED.load(Ordering::Relaxed) {
                    let subc = SUBRE.captures(cmd).unwrap();
                    let (_, [sv1, sv2]) = subc.extract();
                    let v1: i32 = sv1.parse::<_>().unwrap();
                    let v2: i32 = sv2.parse::<_>().unwrap();
                    let product = v1 * v2;
                    println!("{v1} * {v2} = {}", product);
                    Some(product)
                } else {
                    println!("skipping {cmd}");
                    None
                }
            }
        })
        .sum()
}
