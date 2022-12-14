/*
--- Day 8: Treetop Tree House ---

The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.

First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.

The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:

30373
25512
65332
33549
35390

Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.

A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:

    The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
    The top-middle 5 is visible from the top and right.
    The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
    The left-middle 5 is visible, but only from the right.
    The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
    The right-middle 3 is visible from the right.
    In the bottom row, the middle 5 is visible, but the 3 and 4 are not.

With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.

Consider your map; how many trees are visible from outside the grid?


*/

use std::{fs, cmp::max, fmt::{Formatter, Error, self}};

#[derive(PartialEq)]
struct Grove {
    height: usize,
    width: usize,
    grid: Vec<u8>,
}

impl fmt::Debug for Grove {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for x in 0..self.height {
            let offset = x * self.width;
            writeln!(f, "{}", &self.grid[offset..offset+self.width].iter().fold(String::new(), |s, d| s + &d.to_string()))?;
        }
        writeln!(f, "--")
    }
}

fn parse_grid(input: &str) -> Grove {
    let mut grid: Vec<u8> = Vec::new();
    let mut height = 0;
    let mut width = 0;
    for line in input.lines() {
        width = max(width, line.len());
        height += 1;
        grid.append(&mut line.chars().map(|c| c.to_digit(10).unwrap().try_into().expect("no digit?")).collect::<Vec<u8>>());
    }
    Grove {
        height,
        width,
        grid,
    }
}

const LEFT: (isize,isize) = (-1,0);
const RIGHT: (isize,isize) = (1, 0);
const UP: (isize,isize) = (0, -1);
const DOWN: (isize,isize) = (0, 1);

impl Grove {
    fn get_tree(&self, x: usize, y:usize) -> u8 {
        return self.grid[y*self.width+x]
    }

    fn is_visible(&self, x: usize, y:usize) -> bool {
        let this_tree = self.get_tree(x,y);
        // left
        let mut left_visible = true;
        for look_x in (0..=x-1).rev() {
            if self.get_tree(look_x, y) >= this_tree {
                left_visible = false;
                break
            }
        }
        // right
        let mut right_visible = true;
        for look_x in x+1..self.width {
            if self.get_tree(look_x, y) >= this_tree {
                right_visible = false;
                break;
            }
        }
        // up
        let mut up_visible = true;
        for look_y in (0..=y-1).rev() {
            if self.get_tree(x, look_y) >= this_tree {
                up_visible = false;
                break;
            }
        }
        // down
        let mut down_visible = true;
        for look_y in y+1..self.height {
            if self.get_tree(x, look_y) >= this_tree {
                down_visible = false;
                break;
            }
        }
        left_visible | right_visible | up_visible | down_visible
    }

    fn visible_trees(&self) -> usize {
        // Count the outer most ring
        let visible_trees = (2*self.width) + (2*(self.height-2));
        // starting at the next inner ring
        // for each ring up to ring #10
        for ring in 1..9 {
            let mut x = ring;
            let mut y = ring;
        //   for each tree moving clockwise through the ring
            //  first row
            for x_offset in 0..self.width -1 - (ring*2) {
                self.is_visible(x+x_offset, y);
            }
        //     for direction in left,right,up,down
        //       if tree_blocked
        //         break
        //       for i in ring..edge
        //         if tree_in_direction >= this tree
        //           tree_blocked = true
        //           break
        //     if !tree_blocked then increment visible_trees
        }
        visible_trees
    }
}

#[test]
fn test_parse_grid() {
    let test_input = "30373
25512
65332
33549
35390";
    let grid = vec![
        3,0,3,7,3,
        2,5,5,1,2,
        6,5,3,3,2,
        3,3,5,4,9,
        3,5,3,9,0,
    ];
    let height = 5;
    let width = 5;
    let test_grove = Grove {
        height,
        width,
        grid,
    };
    let parsed_grove = parse_grid(test_input);
    println!("{:?}", parsed_grove);
    assert_eq!(test_grove, parsed_grove);
    assert!(parsed_grove.is_visible(1,1));
    assert!(parsed_grove.is_visible(2,1));
    assert!(!parsed_grove.is_visible(3,1));
    assert!(parsed_grove.is_visible(1,2));
    assert!(!parsed_grove.is_visible(2,2));
    assert!(parsed_grove.is_visible(3,2));
    assert!(parsed_grove.is_visible(2,3));
    assert!(!parsed_grove.is_visible(1,3));
    assert!(!parsed_grove.is_visible(3,3));
    assert_eq!(21, parsed_grove.visible_trees());
}

fn main() {
    let buf = fs::read_to_string("2022d8p1.txt").unwrap();
}