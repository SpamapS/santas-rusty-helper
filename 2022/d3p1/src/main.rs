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

--- Part Two ---

As you finish identifying the misplaced items, the Elves come to you with another issue.

For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group. For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves. That is, if a group's badge is item type B, then all three Elves will have item type B somewhere in their rucksack, and at most two of the Elves will be carrying any other item type.

The problem is that someone forgot to put this year's updated authenticity sticker on the badges. All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be attached.

Additionally, nobody wrote down which item type corresponds to each group's badges. The only way to tell which item type is the right one is by finding the one item type that is common between all three Elves in each group.

Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type. So, in the above example, the first group's rucksacks are the first three lines:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg

And the second group's rucksacks are the next three lines:

wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges. In the second group, their badge item type must be Z.

Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (r) for the first group and 52 (Z) for the second group. The sum of these is 70.

Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?


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

fn parse_inventory(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut parsed = Vec::new();
    let mut group_parsed = Vec::new();
    let mut lines = input.lines();
    let mut groupcount = 0;
    let mut groupset = HashSet::new();
    loop {
        let line = lines.next();
        match line {
            None => break,
            Some(line) => {
                let compartment_len = line.len() / 2;
                let left_hs: HashSet<u8, RandomState> = HashSet::from_iter(line[0..compartment_len].bytes());
                let right_hs: HashSet<u8, RandomState> = HashSet::from_iter(line[compartment_len..].bytes());
                let mistake = left_hs.intersection(&right_hs).next().expect(&format!("No dupes for {}", line));
                parsed.push(score_byte(*mistake) as i64);
                if groupcount < 3 {
                    let this_hs: HashSet<u8, RandomState> = HashSet::from_iter(line.bytes());
                    if groupset.len() > 0 {
                        let new_groupset: HashSet<u8, RandomState> = HashSet::from_iter(groupset.intersection(&this_hs).into_iter().map(|b| *b));
                        groupset = new_groupset;
                    } else {
                        groupset = this_hs;
                    }
                    groupcount += 1;
                }
                if groupcount == 3 {
                    if groupset.len() > 1 {
                        panic!("Failed to find one common element at {}", line);
                    }
                    group_parsed.push(score_byte(groupset.clone().into_iter().next().unwrap()) as i64);
                    groupset.clear();
                    groupcount = 0;
                }
            }
        }
    };
    (parsed, group_parsed)
}

#[test]
fn test_inventory() {
    let test_inventory: Vec<i64> = vec![16,38,42,22,20,19];
    let test_group: Vec<i64> = vec![18,52];
    let (parsed_inventory, parsed_groups) = parse_inventory(TEST_INPUT);
    assert_eq!(test_inventory, parsed_inventory);
    assert_eq!(test_group, parsed_groups);
}

fn main() {
    let buf = fs::read_to_string("2022d3p1.txt").unwrap();
    let (parsed_inv, parsed_grp) = parse_inventory(&buf);
    let total: i64 = parsed_inv.iter().sum();
    let group_total: i64 = parsed_grp.iter().sum();
    println!("The total score for this inventory would be: {}", total);
    println!("The total badge score would be: {}", group_total);
}
