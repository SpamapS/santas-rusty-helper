/*
--- Day 4: Giant Squid ---

You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that you can't see any sunlight. What you can see, however, is a giant squid that has attached itself to the outside of your submarine.

Maybe it wants to play bingo?

Bingo is played on a set of boards each consisting of a 5x5 grid of numbers. Numbers are chosen at random, and the chosen number is marked on all boards on which it appears. (Numbers may not appear on all boards.) If all numbers in any row or any column of a board are marked, that board wins. (Diagonals don't count.)

The submarine has a bingo subsystem to help passengers (currently, you and the giant squid) pass the time. It automatically generates a random order in which to draw numbers and a random set of boards (your puzzle input). For example:

7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7

After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no winners, but the boards are marked as follows (shown here adjacent to each other to save space):

22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are still no winners:

22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

Finally, 24 is drawn:

22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

At this point, the third board wins because it has at least one complete row or column of marked numbers (in this case, the entire top row is marked: 14 21 17 24 4).

The score of the winning board can now be calculated. Start by finding the sum of all unmarked numbers on that board; in this case, the sum is 188. Then, multiply that sum by the number that was just called when the board won, 24, to get the final score, 188 * 24 = 4512.

To guarantee victory against the giant squid, figure out which board will win first. What will your final score be if you choose that board?


*/
#![feature(stdin_forwarders,str_split_whitespace_as_str)]
use std::io;
use std::fmt::{Debug, Formatter, Error};

type Row = Vec<Option<u8>>;
struct Board(Vec<Row>);

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for row in self.0.iter() {
            write!(f, "{}\n", String::from("-").repeat(row.len() * 3))?;
            for col in row.iter() {
               match col {
                   None => write!(f, " X ")?,
                   Some(v) => write!(f, "{:3}", v)?,
               };
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    println!("Paste bingo output:");
    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line).unwrap();
    io::stdin().read_line(&mut String::new()).unwrap(); // throw one away;

    let called_numbers: Vec<u8> = first_line.split(",").map(|n_str| {n_str.trim().parse::<u8>().unwrap()}).collect();
    let mut boards: Vec<Board> = Vec::new();
    let mut this_board: Board = Board(Vec::new());
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.trim().len() > 0 {
            let this_row: Row = line.split_whitespace().map(|n_str| {Some(n_str.parse::<u8>().unwrap())}).collect();
            this_board.0.push(this_row);
        } else {
            println!("---");
            println!("{:?}", this_board);
            boards.push(this_board);
            this_board = Board(Vec::new());
        }
    }
    
    for called_num in called_numbers.iter() {
        let mut boardno = 0;
        for mut board in boards.iter_mut() {
            mark_board(&mut board, *called_num);
            match check_board(&board, *called_num) {
                None => (),
                Some(answer) => {
                    println!("{:?}", board);
                    println!("We found it! Board #{} wins with {}, answer: {}", boardno, called_num, answer);
                    return;
                }
            }
            boardno += 1;
        }
    }
    panic!("No winners?!");
}

fn mark_board(board: &mut Board, n: u8){
    for row in board.0.iter_mut() {
        for my_n in row.iter_mut() {
            if match my_n {
                Some(nval) => *nval == n,
                None => false,
            } {
                *my_n = None;
                return
            }
        }
    }
}

fn sum_board(board: &Board) -> u32 {
    return board.0.iter()
        .flatten()
        .filter_map(|v| {
            match v {
                None => None,
                Some(v) => Some(u32::from(*v)),
            }
        }).sum()
}

fn check_board(board: &Board, called_num: u8) -> Option<u32> {
    for row in board.0.iter() {
        if row.iter().all(|x| x.is_none()) {
            return Some(sum_board(board) * u32::from(called_num))
        }
    }
    for col in 0..board.0[0].len() {
        if board.0.iter().map(|row| row[col]).all(|v| v.is_none()) {
            return Some(sum_board(board) * u32::from(called_num))
        }
    }
    return None;
}