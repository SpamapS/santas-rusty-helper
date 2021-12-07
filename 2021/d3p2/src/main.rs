/*
--- Part Two ---

Next, you should verify the life support rating, which can be determined by multiplying the oxygen generator rating by the CO2 scrubber rating.

Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in your diagnostic report - finding them is the tricky part. Both values are located using a similar process that involves filtering out values until only one remains. Before searching for either rating value, start with the full list of binary numbers from your diagnostic report and consider just the first bit of those numbers. Then:

    Keep only numbers selected by the bit criteria for the type of rating value for which you are searching. Discard numbers which do not match the bit criteria.
    If you only have one number left, stop; this is the rating value for which you are searching.
    Otherwise, repeat the process, considering the next bit to the right.

The bit criteria depends on which type of rating value you want to find:

    To find oxygen generator rating, determine the most common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 1 in the position being considered.
    To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 0 in the position being considered.

For example, to determine the oxygen generator rating value using the same example diagnostic report from above:

    Start with all 12 numbers and consider only the first bit of each number. There are more 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the first position: 11110, 10110, 10111, 10101, 11100, 10000, and 11001.
    Then, consider the second bit of the 7 remaining numbers: there are more 0 bits (4) than 1 bits (3), so keep only the 4 numbers with a 0 in the second position: 10110, 10111, 10101, and 10000.
    In the third position, three of the four numbers have a 1, so keep those three: 10110, 10111, and 10101.
    In the fourth position, two of the three numbers have a 1, so keep those two: 10110 and 10111.
    In the fifth position, there are an equal number of 0 bits and 1 bits (one each). So, to find the oxygen generator rating, keep the number with a 1 in that position: 10111.
    As there is only one number left, stop; the oxygen generator rating is 10111, or 23 in decimal.

Then, to determine the CO2 scrubber rating value from the same example above:

    Start again with all 12 numbers and consider only the first bit of each number. There are fewer 0 bits (5) than 1 bits (7), so keep only the 5 numbers with a 0 in the first position: 00100, 01111, 00111, 00010, and 01010.
    Then, consider the second bit of the 5 remaining numbers: there are fewer 1 bits (2) than 0 bits (3), so keep only the 2 numbers with a 1 in the second position: 01111 and 01010.
    In the third position, there are an equal number of 0 bits and 1 bits (one each). So, to find the CO2 scrubber rating, keep the number with a 0 in that position: 01010.
    As there is only one number left, stop; the CO2 scrubber rating is 01010, or 10 in decimal.

Finally, to find the life support rating, multiply the oxygen generator rating (23) by the CO2 scrubber rating (10) to get 230.

Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and CO2 scrubber rating, then multiply them together. What is the life support rating of the submarine? (Be sure to represent your answer in decimal, not binary.)


*/
#![feature(stdin_forwarders)]
use std::io;

fn main() {
    println!("Enter submarine diagnostic report:");
    let dlines: Vec<String> = io::stdin().lines().map(|s| s.unwrap()).collect();
    let digits: Vec<u16> = dlines.iter().map(|s| {u16::from_str_radix(s.trim(), 2).unwrap()}).collect();
    let oxygen_rating = bitcrit(&digits, 15, |ones, zeroes| {ones >= zeroes});
    let co2_rating = bitcrit(&digits, 15, |ones, zeroes| {ones > 0 && ones < zeroes});
    let lifesupport_rating: u32 = u32::from(oxygen_rating) * u32::from(co2_rating);
    println!("oxygen_rating = {} co2_rating = {}", oxygen_rating, co2_rating);
    println!("life support rating = {}", lifesupport_rating);
}

/* counts 1's at pos in all digits, then passes the # of times 1 was found and the # of elements searched to cmp. if cmp returns true, it keeps all the digits with 1's, otherwise it keeps all the 0's. */
fn bitcrit<F>(digits: &Vec<u16>, pos: usize, cmp: F) -> u16 
where F: Fn(usize, usize) -> bool
{
    //println!("{} pos = {}", " ".repeat(pos), pos);
    let mut ones: Vec<u16> = Vec::new();
    let mut zeroes: Vec<u16> = Vec::new();
    let mask = 1 << pos;
    for digit in digits {
        if digit & mask == mask {
            ones.push(*digit);
        } else {
            zeroes.push(*digit);
        }
    }
    /*
    println!("ones   = {:?}", ones.iter().map(|n| {format!("{:05b} ", n)}).collect::<String>());
    println!("zeroes = {:?}", zeroes.iter().map(|n| {format!("{:05b} ", n)}).collect::<String>());
    */
    
    let matches = match cmp(ones.len(), zeroes.len()) {
        true => ones,
        false => zeroes,
    };
    if matches.len() == 1 {
        return matches[0];
    }
    if pos == 0 {
        panic!("Maximum bit width exceeded");
    }
    return bitcrit(&matches, pos-1, cmp);
}
