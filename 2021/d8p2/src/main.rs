/*
--- Day 8: Seven Segment Search ---

You barely reach the safety of the cave when the whale smashes into the cave mouth, collapsing it. Sensors indicate another exit to this cave at a much greater depth, so you have no choice but to press on.

As your submarine slowly makes its way through the cave system, you notice that the four-digit seven-segment displays in your submarine are malfunctioning; they must have been damaged during the escape. You'll be in a lot of trouble without them, so you'd better figure out what's wrong.

Each digit of a seven-segment display is rendered by turning on or off any of seven segments named a through g:

  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

So, to render a 1, only segments c and f would be turned on; the rest would be off. To render a 7, only segments a, c, and f would be turned on.

The problem is that the signals which control the segments have been mixed up on each display. The submarine is still trying to display numbers by producing output on signal wires a through g, but those wires are connected to segments randomly. Worse, the wire/segment connections are mixed up separately for each four-digit display! (All of the digits within a display use the same connections, though.)

So, you might know that only signal wires b and g are turned on, but that doesn't mean segments b and g are turned on: the only digit that uses two segments is 1, so it must mean segments c and f are meant to be on. With just that information, you still can't tell which wire (b/g) goes to which segment (c/f). For that, you'll need to collect more information.

For each display, you watch the changing signals for a while, make a note of all ten unique signal patterns you see, and then write down a single four digit output value (your puzzle input). Using the signal patterns, you should be able to work out which pattern corresponds to which digit.

For example, here is what you might see in a single entry in your notes:

acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf

(The entry is wrapped here to two lines so it fits; in your notes, it will all be on a single line.)

Each entry consists of ten unique signal patterns, a | delimiter, and finally the four digit output value. Within an entry, the same wire/segment connections are used (but you don't know what the connections actually are). The unique signal patterns correspond to the ten different ways the submarine tries to render a digit using the current wire/segment connections. Because 7 is the only digit that uses three segments, dab in the above example means that to render a 7, signal lines d, a, and b are on. Because 4 is the only digit that uses four segments, eafb means that to render a 4, signal lines e, a, f, and b are on.

Using this information, you should be able to work out which combination of signal wires corresponds to each of the ten digits. Then, you can decode the four digit output value. Unfortunately, in the above example, all of the digits in the output value (cdfeb fcadb cdfeb cdbaf) use five segments and are more difficult to deduce.

For now, focus on the easy digits. Consider this larger example:

be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
fgae cfgab fg bagce

Because the digits 1, 4, 7, and 8 each use a unique number of segments, you should be able to tell which combinations of signals correspond to those digits. Counting only digits in the output values (the part after | on each line), in the above example, there are 26 instances of digits that use a unique number of segments (highlighted above).

In the output values, how many times do digits 1, 4, 7, or 8 appear?

--- Part Two ---

Through a little deduction, you should now be able to determine the remaining digits. Consider again the first example above:

acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf

After some careful analysis, the mapping between signal wires and segments only make sense in the following configuration:

 dddd
e    a
e    a
 ffff
g    b
g    b
 cccc

So, the unique signal patterns would correspond to the following digits:

    acedgfb: 8
    cdfbe: 5
    gcdfa: 2
    fbcad: 3
    dab: 7
    cefabd: 9
    cdfgeb: 6
    eafb: 4
    cagedb: 0
    ab: 1

Then, the four digits of the output value can be decoded:

    cdfeb: 5
    fcadb: 3
    cdfeb: 5
    cdbaf: 3

Therefore, the output value for this entry is 5353.

Following this same process for each entry in the second, larger example above, the output value of each entry can be determined:

    fdgacbe cefdb cefbgd gcbe: 8394
    fcgedb cgb dgebacf gc: 9781
    cg cg fdcagb cbg: 1197
    efabcd cedba gadfec cb: 9361
    gecf egdcabf bgf bfgea: 4873
    gebdcfa ecba ca fadegcb: 8418
    cefg dcbef fcge gbcadfe: 4548
    ed bcgafe cdgba cbgef: 1625
    gbdfcae bgc cg cgb: 8717
    fgae cfgab fg bagce: 4315

Adding all of the output values in this larger example produces 61229.

For each entry, determine all of the wire/segment connections and decode the four-digit output values. What do you get if you add up all of the output values?


*/
#![feature(stdin_forwarders, int_abs_diff)]
use std::io;
use std::collections::{HashMap, HashSet};

use regex::Regex;

const PATTERNS: [&str;10] = ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];
const N_SEGMENTS: usize = 7;

fn main() {
    let mut unique_lens: HashMap<usize, usize> = HashMap::new();
    unique_lens.insert(2, 1);
    unique_lens.insert(4, 4);
    unique_lens.insert(3, 7);
    unique_lens.insert(7, 8);
    println!("Input notes:");
    for inputline in io::stdin().lines() {
        let inputline = inputline.unwrap();
        println!("{}", inputline);
        let parts: Vec<&str> = inputline.split("|").collect();
        let signals: Vec<&str> = parts[0].trim().split(" ").collect();
        let scrambled_digits = parts[1].trim().split(" ");
        let mut segment_map: Vec<Option<String>> = vec![None;10];
        let mut known_sorted_signals: HashMap<String, String> = HashMap::new();
        // digits will reveal some maps
        for scrambled_digit in scrambled_digits.clone() {
            let mut sorted_scrambled_digit: Vec<char> = scrambled_digit.chars().collect();
            sorted_scrambled_digit.sort();
            let sorted_scrambled_digit = String::from_iter(sorted_scrambled_digit);
            match unique_lens.get(&scrambled_digit.len()) {
                None => (),
                Some(digit) => segment_map[*digit] = Some(sorted_scrambled_digit),
            }
        }
        // Any signals with the same lens as those we found?
        let mut misses: Vec<String> = Vec::new();
        for scrambled_signal in signals.iter() {
            let mut sorted_scrambled_signal: Vec<char> = scrambled_signal.chars().collect();
            sorted_scrambled_signal.sort();
            let sorted_scrambled_signal = String::from_iter(sorted_scrambled_signal);
            match unique_lens.get(&scrambled_signal.len()) {
                None => misses.push(scrambled_signal.to_string()),
                Some(digit) => match &segment_map[*digit] {
                    None => misses.push(scrambled_signal.to_string()),
                    Some(sorted_scrambled_digit) => {known_sorted_signals.insert(sorted_scrambled_signal, sorted_scrambled_digit.clone());},
                }
            }
        }
        // Now find signals which have all of the chars of a known signal contained within. Partially fill a digit with them, and see if
        // there is only 1 digit pattern that matches, if so, add it to the maps.
        for round in 0..misses.len() {
            let mut more_misses = Vec::new();
            for scrambled_signal in misses.drain(..) {
                let mut sorted_scrambled_signal: Vec<char> = scrambled_signal.chars().collect();
                sorted_scrambled_signal.sort();
                let sorted_scrambled_signal = String::from_iter(sorted_scrambled_signal);
                let mut all_matches: Vec<(String, String)> = Vec::new();
                let mut found = false;
                for (sorted_known_signal, sorted_known_digit) in known_sorted_signals.iter() {
                    if sorted_known_digit.chars().all(|c| sorted_scrambled_signal.contains(c)) {
                        // Potential match! Find known digits which match this
                        let matches: Vec<String> = Vec::from(PATTERNS).iter()
                            .map(|pattern| String::from(*pattern))
                            .filter(|pattern| sorted_known_digit.chars().all(|c| pattern.contains(c)) && pattern.len() == scrambled_signal.len())
                            .collect();
                        if matches.len() == 1 {
                            all_matches.push((sorted_scrambled_signal.clone(), matches[0].clone()));
                            //println!("Fist order match for {} with {}", scrambled_signal, sorted_known_digit);
                            found = true;
                            break;
                        }
                    }
                    if sorted_scrambled_signal.chars().all(|c| sorted_known_signal.contains(c)) {
                        // All of the chars in our signal, are in a known signal, so we can at least know all
                        // of the digits that will light up. Let's see if *those* contain a known digit of the same length
                        //println!("Second order match for {} on {} with {}", scrambled_signal, sorted_known_signal, sorted_known_digit);
                        let matches: Vec<String> = Vec::from(PATTERNS).iter()
                            .map(|pattern| String::from(*pattern))
                            //.map(|pattern| { if pattern.len() == scrambled_signal.len() {println!("left = {} right = {}", pattern, sorted_known_digit)}; pattern})
                            .filter(|pattern| pattern.len() == scrambled_signal.len() && pattern.chars().all(|c| sorted_known_digit.contains(c)))
                            .collect();
                        //println!("Matches = {:?}", matches);
                        if matches.len() == 1 {
                            //println!("Found {} is the only one for {}", sorted_scrambled_signal, matches[0]);
                            all_matches.push((sorted_scrambled_signal.clone(), matches[0].clone()));
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    more_misses.push(scrambled_signal.clone())
                }
                for (p, v) in all_matches {
                    known_sorted_signals.insert(p, v);
                }
            }
            if more_misses.len() == 0 {
                break
            }
            if more_misses.len() == 1 {
                // There's only one left!
                let found_patterns: HashSet<&String> = known_sorted_signals.values().collect();
                let mut final_pattern = PATTERNS.iter().filter(|x| found_patterns.contains(&x.to_string())).map(|s| s.to_string());
                known_sorted_signals.insert(more_misses.pop().unwrap(), final_pattern.next().unwrap());
                break
            }
            println!("[{:?}] At round {} we have these misses: {:?}", misses, round, more_misses);
            misses.extend(more_misses.clone());
            more_misses.truncate(0);
        }
        println!("known_sorted-signals = {:?}", known_sorted_signals);
        // Make the digits
        let mut digit_total = 0;
        let mut elevator:usize  = 1000;
        let patterns = Vec::from(PATTERNS);
        for scrambled_digit in scrambled_digits {
            let digit_value = match unique_lens.get(&scrambled_digit.len()) {
                None => {
                    let mut sorted_scrambled_digit: Vec<char> = scrambled_digit.chars().collect();
                    sorted_scrambled_digit.sort();
                    let sorted_scrambled_digit = String::from_iter(sorted_scrambled_digit);
                    let sorted_signal = known_sorted_signals.get(&sorted_scrambled_digit).unwrap();
                    println!("Looking for {} in {:?}", sorted_signal, patterns);
                    patterns.iter().position(|s| s == &sorted_signal).unwrap()
                },
                Some(digit) => *digit,
            };
            digit_total += digit_value * elevator;
            elevator = elevator / 10;
        }
        println!("final = {}", digit_total);
    }
}