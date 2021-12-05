/*
--- Day 2: Dive! ---

Now, you need to figure out how to pilot this thing.

It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:

    forward X increases the horizontal position by X units.
    down X increases the depth by X units.
    up X decreases the depth by X units.

Note that since you're on a submarine, down and up affect your depth, and so they have the opposite result of what you might expect.

The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:

forward 5
down 5
forward 8
up 3
down 8
forward 2

Your horizontal position and depth both start at 0. The steps above would then modify them as follows:

    forward 5 adds 5 to your horizontal position, a total of 5.
    down 5 adds 5 to your depth, resulting in a value of 5.
    forward 8 adds 8 to your horizontal position, a total of 13.
    up 3 decreases your depth by 3, resulting in a value of 2.
    down 8 adds 8 to your depth, resulting in a value of 10.
    forward 2 adds 2 to your horizontal position, a total of 15.

After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)

Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?


*/
#![feature(stdin_forwarders)]
use std::io;

fn main() {
    let mut depth = 0;
    let mut pos = 0;
    let mut lc = 0;
    println!("Enter submarine commands:");
    for line in io::stdin().lines() {
        lc += 1;
        let line = line.unwrap();
        let args: Vec<&str> = line.split(" ").collect();
        let command = args[0];
        let amt: i32 = args[1].parse().unwrap();
        match command {
            "up" => depth -= amt,
            "down" => depth += amt,
            "forward" => pos += amt,
            _ => panic!("Invalid command ({}) at line #{}", command, lc)
        }
        println!("We are now at a depth of {}, position {}, mult = {}", depth, pos, depth*pos);
    }
}
 