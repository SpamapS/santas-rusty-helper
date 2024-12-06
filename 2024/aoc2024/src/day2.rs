/*
--- Day 2: Red-Nosed Reports ---

Fortunately, the first location The Historians want to search isn't a long walk from the Chief Historian's office.

While the Red-Nosed Reindeer nuclear fusion/fission plant appears to contain no sign of the Chief Historian, the engineers there run up to you as soon as they see you. Apparently, they still talk about the time Rudolph was saved through molecular synthesis from a single electron.

They're quick to add that - since you're already here - they'd really appreciate your help analyzing some unusual data from the Red-Nosed reactor. You turn to check if The Historians are waiting for you, but they seem to have already divided into groups that are currently searching every corner of the facility. You offer to help with the unusual data.

The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are separated by spaces. For example:

7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9

This example data contains six reports each containing five levels.

The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

    The levels are either all increasing or all decreasing.
    Any two adjacent levels differ by at least one and at most three.

In the example above, the reports can be found safe or unsafe by checking those rules:

    7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
    1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
    9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
    1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
    8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
    1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.

So, in this example, 2 reports are safe.

Analyze the unusual data from the engineers. How many reports are safe?
 */

use std::{fs::File, path::PathBuf};

use crate::util::parse_file;

pub fn day2(path: Option<PathBuf>) {
    let input = match path {
        None => File::open("day2test.txt"),
        Some(path) => File::open(path),
    };
    let nums = parse_file(input.unwrap(), parser);
    println!(
        "{} are safe without dampener",
        nums.iter().filter(|&report| issafe(report)).count()
    );
    println!(
        "{} are safe with dampener",
        nums.iter()
            .filter(|&report| dampener_issafe(report))
            .count()
    );
}

fn dampener_issafe(report: &Vec<i32>) -> bool {
    if issafe(report) {
        println!("no dampener required");
        return true;
    }
    (0..report.len()).any(|i| {
        println!("  checking {:?} without {i}", report);
        issafe(
            &report
                .iter()
                .enumerate()
                .filter(|(offset, _level)| *offset != i)
                .map(|(_offset, level)| *level)
                .collect(),
        )
    })
}

fn issafe(report: &Vec<i32>) -> bool {
    struct Track {
        last: Option<i32>,
        last_diff: Option<i32>,
    }
    let acc = Track {
        last: None,
        last_diff: None,
    };
    report
        .iter()
        .scan(acc, |acc, level| match acc.last {
            None => {
                acc.last = Some(*level);
                Some(level)
            }
            Some(last) => {
                acc.last = Some(*level);
                let diff: i32 = (last - level).try_into().unwrap();
                if diff.abs() > 3 {
                    println!("{last} to {level} not safe because spike ({diff})");
                    None
                } else {
                    match acc.last_diff {
                        None => {
                            acc.last_diff = Some(diff);
                            println!("{last} to {level} safe because no previous ");
                            Some(level)
                        }
                        Some(last_diff) => {
                            acc.last_diff = Some(diff);
                            if last_diff < 0 && diff < 0 {
                                println!("{last} to {level} safe because both diffs negative {last_diff} -> {diff}");
                                Some(level)
                            } else if last_diff > 0 && diff > 0 {
                                println!("{last} to {level} safe because both diffs positive {last_diff} -> {diff}");
                                Some(level)
                            } else {
                                println!("{last} to {level} not safe because wrong direction? {last_diff} -> {diff}");
                                None
                            }
                        }
                    }
                }
            }
        })
        .count()
        == report.len()
}

fn parser(line: String) -> Vec<i32> {
    let parts = line.split(" ");
    parts
        .map(|p| p.parse::<i32>().expect("all parts should be ints"))
        .collect()
}
