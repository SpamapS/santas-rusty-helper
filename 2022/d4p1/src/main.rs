/*
--- Day 4: Camp Cleanup ---

Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp. Every section has a unique ID number, and each Elf is assigned a range of section IDs.

However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments overlap. To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).

For example, consider the following list of section assignment pairs:

2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8

For the first few pairs, this list means:

    Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
    The Elves in the second pair were each assigned two sections.
    The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.

This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:

.234.....  2-4
.....678.  6-8

.23......  2-3
...45....  4-5

....567..  5-7
......789  7-9

.2345678.  2-8
..34567..  3-7

.....6...  6-6
...456...  4-6

.23456...  2-6
...45678.  4-8

Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.

In how many assignment pairs does one range fully contain the other?


*/

use std::collections::hash_map::RandomState;
use std::error::Error;
use std::fs;
use std::collections::HashSet;
use std::num::ParseIntError;

const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

fn parse_sections(input: &str) -> Result<Vec<((i32, i32), (i32, i32))>, ParseIntError> {
    let mut parsed = Vec::new();
    for line in input.lines() {
        let (left, right) = line.split_once(',').expect("No comma in line");
        let (lmin, lmax) = left.split_once('-').expect("No - in left side");
        let (rmin, rmax) = right.split_once('-').expect("No - in right side");
        let lmin: i32 = lmin.parse()?;
        let lmax: i32 = lmax.parse()?;
        let rmin: i32 = rmin.parse()?;
        let rmax: i32 = rmax.parse()?;
        parsed.push(((lmin, lmax), (rmin, rmax)));
    }
    Ok(parsed)
}

fn count_overlaps(sections: Vec<((i32,i32),(i32,i32))>) -> i32 {
    sections.iter()
        .fold(0, |n, ((lmin, lmax),(rmin, rmax))| {
           n + ((rmin <= lmin && rmax >= lmax) || (lmin <= rmin && lmax >= rmax)) as i32 
        })
}

#[test]
fn test_inventory() {
    let test_sections: Vec<((i32, i32),(i32,i32))> = vec![
        ((2,4),(6,8)),
        ((2,3),(4,5)),
        ((5,7),(7,9)),
        ((2,8),(3,7)),
        ((6,6),(4,6)),
        ((2,6),(4,8)),
    ];
    let parsed_sections = parse_sections(TEST_INPUT).unwrap();
    assert_eq!(test_sections, parsed_sections);
    assert_eq!(2, count_overlaps(parsed_sections));
}

fn main() {
    let buf = fs::read_to_string("2022d4p1.txt").unwrap();
    let parsed_sections = parse_sections(&buf).unwrap();
    println!("There are {} fully overlapping pairs of sections.", count_overlaps(parsed_sections));
}
