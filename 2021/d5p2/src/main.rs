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

--- Part Two ---

Unfortunately, considering only horizontal and vertical lines doesn't give you the full picture; you need to also consider diagonal lines.

Because of the limits of the hydrothermal vent mapping system, the lines in your list will only ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:

    An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
    An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.

Considering all lines from the above example would now produce the following diagram:

1.1....11.
.111...2..
..2.1.111.
...1.2.2..
.112313211
...1.2....
..1...1...
.1.....1..
1.......1.
222111....

You still need to determine the number of points where at least two lines overlap. In the above example, this is still anywhere in the diagram with a 2 or larger - now a total of 12 points.

Consider all of the lines. At how many points do at least two lines overlap?


*/
#![feature(stdin_forwarders, str_split_whitespace_as_str)]
use std::cmp::{max, min};
use std::fmt::{Debug, Error, Formatter};
use std::io;

type Row = Vec<i32>;
struct Board(Vec<Row>);
#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct VentLine {
    begin: Coord,
    end: Coord,
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut j = 0;
        for row in self.0.iter() {
            write!(f, "{:06} ", j);
            j += 1;
            for col in row.iter() {
                write!(
                    f,
                    "{}",
                    match col {
                        0 => String::from("."),
                        n => format!("{}", n),
                    }
                )?
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
    let vent_lines: Vec<VentLine> = io::stdin()
        .lines()
        .map(|line| {
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
        })
        .collect();
    // Make the board
    let mut board = Board(Vec::new());
    for _row in 0..max_y + 1 {
        board.0.push((0..max_x + 2).map(|_| 0).collect::<Row>());
    }
    //println!("{:?}", board);
    println!("Board size is {}x{}", max_x + 1, max_y + 1);
    println!("Lens are {}x{}", board.0.len(), board.0[0].len());
    // Draw lines
    for vent_line in vent_lines.iter() {
        let start_x = vent_line.begin.x;
        let end_x = vent_line.end.x;
        let start_y = vent_line.begin.y;
        let end_y = vent_line.end.y;
        let mut row = start_x;
        let mut col = start_y;
        let row_step = match end_x - start_x {
            n if n == 0 => 0,
            n if n > 0 => 1,
            _ => -1,
        };
        let col_step = match end_y - start_y {
            n if n == 0 => 0,
            n if n > 0 => 1,
            _ => -1,
        };
        loop {
            if row as usize == board.0.len() || col as usize == board.0[row as usize].len() {
                println!(
                    "{:?} row={} col={} row_step={} col_step={}",
                    vent_line, row, col, row_step, col_step
                );
            }
            board.0[row as usize][col as usize] += 1;
            row += row_step;
            col += col_step;
            if row == end_x + row_step && col == end_y + col_step {
                break;
            }
        }
        //println!("{:?}", board);
    }
    // Print board
    //println!("{:?}", board);
    let answer = board.0.iter().flatten().filter(|&x| *x >= 2).count();
    println!("answer = {:?}", answer);
}

fn parse_tuple(txt: &str) -> Coord {
    let ints: Vec<i32> = txt
        .replace(&['(', ')'][..], "")
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    Coord {
        x: ints[0],
        y: ints[1],
    }
}
