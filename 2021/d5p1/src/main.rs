/*
--- Day 5: Hydrothermal Venture ---

You come across a field of hydrothermal vents on the ocean floor! These vents constantly produce large, opaque clouds, so it would be best to avoid them if possible.

They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents (your puzzle input) for you to review. For example:

0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2

Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are the coordinates of one end the line segment and x2,y2 are the coordinates of the other end. These line segments include the points at both ends. In other words:

    An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
    An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.

For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.

So, the horizontal and vertical lines from the above list would produce the following diagram:

.......1..
..1....1..
..1....1..
.......1..
.112111211
..........
..........
..........
..........
222111....

In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9. Each position is shown as the number of lines which cover that point or . if no line covers that point. The top-left pair of 1s, for example, comes from 2,2 -> 2,1; the very bottom row is formed by the overlapping lines 0,9 -> 5,9 and 0,9 -> 2,9.

To avoid the most dangerous areas, you need to determine the number of points where at least two lines overlap. In the above example, this is anywhere in the diagram with a 2 or larger - a total of 5 points.

Consider only horizontal and vertical lines. At how many points do at least two lines overlap?

*/
#![feature(stdin_forwarders,str_split_whitespace_as_str)]
use std::io;
use std::fmt::{Debug, Formatter, Error};
use std::cmp::{min,max};

type Row = Vec<u16>;
struct Board(Vec<Row>);
#[derive(Debug)]
struct Coord {
    x: u16,
    y: u16,
}
#[derive(Debug)]
struct VentLine {
    begin: Coord,
    end: Coord,
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for row in self.0.iter() {
            //write!(f, "{}\n", String::from("-").repeat(row.len() * 3))?;
            for col in row.iter() {
               write!(f, "{}", match col { 0 => String::from("."), n => format!("{}",n)})?
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    println!("Paste vent report:");
    let mut max_x = 0;
    let mut max_y = 0;
    let vent_lines: Vec<VentLine> = io::stdin().lines().map(|line| {
        let line = line.unwrap();
        let mut tuples = line.split(" -> ");
        let begin = parse_tuple(tuples.next().unwrap());
        max_x = max(begin.x, max_x);
        max_y = max(begin.y, max_y);
        let end = parse_tuple(tuples.next().unwrap());
        max_x = max(end.x, max_x);
        max_y = max(end.y, max_y);
        VentLine {
            begin: begin,
            end: end,
        }
    }).collect();
    // Make the board
    let mut board = Board(Vec::new());
    for _row in 0..max_y+1 {
        board.0.push((0..max_x+1).map(|_| 0).collect::<Row>());
    }
    println!("{:?}", board);
    // Draw lines
    for vent_line in vent_lines.iter() {
        if vent_line.begin.x == vent_line.end.x {
            println!("{:?} is vertical", vent_line);
            let start_y = min(vent_line.begin.y, vent_line.end.y);
            let end_y = max(vent_line.begin.y, vent_line.end.y)+1;
            for row in start_y..end_y {
                board.0[row as usize][vent_line.begin.x as usize] += 1
            }
        } else if vent_line.begin.y == vent_line.end.y {
            println!("{:?} is horizontal", vent_line);
            let start_x = min(vent_line.begin.x, vent_line.end.x);
            let end_x = max(vent_line.begin.x, vent_line.end.x)+1;
            for col in start_x..end_x {
                board.0[vent_line.begin.y as usize][col as usize] += 1
            }
        } else {
            println!("{:?} is diagonal", vent_line);
        }
    }
    // Print board
    println!("{:?}", board);
    let answer = board.0.iter().flatten().filter(|&x| *x >= 2).count();
    println!("answer = {:?}", answer);
}

fn parse_tuple(txt: &str) -> Coord {
    let ints: Vec<u16> = txt.replace(&['(',')'][..], "")
                      .split(",")
                      .map(|s| s.parse::<u16>().unwrap())
                      .collect();
    Coord {
        x: ints[0],
        y: ints[1]
    }
}