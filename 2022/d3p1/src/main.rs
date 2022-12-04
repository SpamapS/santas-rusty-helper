/*
--- Day 3: Rucksack Reorganization ---

One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey.
Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need
to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to go into exactly
one of the two compartments. The Elf that did the packing failed to follow this rule for exactly
one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but
they need your help finding the errors. Every item type is identified by a single lowercase or
uppercase letter (that is, a and A refer to different types of items).

The list of items for each rucksack is given as characters all on a single line. A given rucksack
always has the same number of items in each of its two compartments, so the first half of the
characters represent items in the first compartment, while the second half of the characters
represent items in the second compartment.

For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

    The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first
    compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items
    hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
    The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item
    type that appears in both compartments is uppercase L.
    The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is
    uppercase P.
    The fourth rucksack's compartments only share item type v.
    The fifth rucksack's compartments only share item type t.
    The sixth rucksack's compartments only share item type s.

To help prioritize item rearrangement, every item type can be converted to a priority:

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

In the above example, the priority of the item type that appears in both compartments of each
rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

Find the item type that appears in both compartments of each rucksack. What is the sum of the
priorities of those item types?

*/

use std::collections::hash_map::RandomState;
use std::fs;
use std::collections::HashSet;

const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

fn score_byte(b: u8) -> u8 {
    // lowercase
    if b >= 97 && b <= 122 {
        return b - 96
    }
    // uppercase
    if b >= 65 && b <= 90 {
        return b - 38
    }
    panic!("Unknown score for byte = {}", b);
}

fn parse_inventory(input: &str) -> Vec<i64> {
    let mut parsed = Vec::new();
    let lines = input.lines();
    lines.for_each(|line| {
        let compartment_len = line.len() / 2;
        let left_hs: HashSet<u8, RandomState> = HashSet::from_iter(line[0..compartment_len].bytes());
        let right_hs: HashSet<u8, RandomState> = HashSet::from_iter(line[compartment_len..].bytes());
        let mistake = left_hs.intersection(&right_hs).next().expect(&format!("No dupes for {}", line));
        parsed.push(score_byte(*mistake) as i64);
    });
    parsed
}

#[test]
fn test_inventory() {
    let test_inventory: Vec<i64> = vec![16,38,42,22,20,19];
    let parsed_inventory = parse_inventory(TEST_INPUT);
    assert_eq!(test_inventory, parsed_inventory);
}

fn main() {
    let buf = fs::read_to_string("2022d3p1.txt").unwrap();
    let parsed_inv: Vec<i64> = parse_inventory(&buf);
    let total: i64 = parsed_inv.iter().sum();
    println!("The total score for this inventory would be: {}", total);
}
