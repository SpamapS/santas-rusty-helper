/*
--- Day 9: Rope Bridge ---

This rope bridge creaks as you walk along it. You aren't sure how old it is, or whether it can even support your weight.

It seems to support the Elves just fine, though. The bridge spans a gorge which was carved out by the massive river far below you.

You step carefully; as you do, the ropes stretch and twist. You decide to distract yourself by modeling rope physics; maybe you can even figure out where not to step.

Consider a rope with a knot at each end; these knots mark the head and the tail of the rope. If the head moves far enough away from the tail, the tail is pulled toward the head.

Due to nebulous reasoning involving Planck lengths, you should be able to model the positions of the knots on a two-dimensional grid. Then, by following a hypothetical series of motions (your puzzle input) for the head, you can determine how the tail will move.

Due to the aforementioned Planck lengths, the rope must be quite short; in fact, the head (H) and tail (T) must always be touching (diagonally adjacent and even overlapping both count as touching):

....
.TH.
....

....
.H..
..T.
....

...
.H. (H covers T)
...

If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough:

.....    .....    .....
.TH.. -> .T.H. -> ..TH.
.....    .....    .....

...    ...    ...
.T.    .T.    ...
.H. -> ... -> .T.
...    .H.    .H.
...    ...    ...

Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up:

.....    .....    .....
.....    ..H..    ..H..
..H.. -> ..... -> ..T..
.T...    .T...    .....
.....    .....    .....

.....    .....    .....
.....    .....    .....
..H.. -> ...H. -> ..TH.
.T...    .T...    .....
.....    .....    .....

You just need to work out where the tail goes as the head follows a series of motions. Assume the head and the tail both start at the same position, overlapping.

For example:

R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2

This series of motions moves the head right four steps, then up four steps, then left three steps, then down one step, and so on. After each step, you'll need to update the position of the tail if the step means the head is no longer adjacent to the tail. Visually, these motions occur as follows (s marks the starting position as a reference point):

== Initial State ==

......
......
......
......
H.....  (H covers T, s)

== R 4 ==

......
......
......
......
TH....  (T covers s)

......
......
......
......
sTH...

......
......
......
......
s.TH..

......
......
......
......
s..TH.

== U 4 ==

......
......
......
....H.
s..T..

......
......
....H.
....T.
s.....

......
....H.
....T.
......
s.....

....H.
....T.
......
......
s.....

== L 3 ==

...H..
....T.
......
......
s.....

..HT..
......
......
......
s.....

.HT...
......
......
......
s.....

== D 1 ==

..T...
.H....
......
......
s.....

== R 4 ==

..T...
..H...
......
......
s.....

..T...
...H..
......
......
s.....

......
...TH.
......
......
s.....

......
....TH
......
......
s.....

== D 1 ==

......
....T.
.....H
......
s.....

== L 5 ==

......
....T.
....H.
......
s.....

......
....T.
...H..
......
s.....

......
......
..HT..
......
s.....

......
......
.HT...
......
s.....

......
......
HT....
......
s.....

== R 2 ==

......
......
.H....  (H covers T)
......
s.....

......
......
.TH...
......
s.....

After simulating the rope, you can count up all of the positions the tail visited at least once. In this diagram, s again marks the starting position (which the tail also visited) and # marks other positions the tail visited:

..##..
...##.
.####.
....#.
s###..

So, there are 13 positions the tail visited at least once.

Simulate your complete hypothetical series of motions. How many positions does the tail of the rope visit at least once?

*/

use std::{fs, collections::HashSet};

#[derive(Debug)]
struct Rope {
    head: Coordinates,
    tail: Coordinates,
    tail_positions: HashSet<Coordinates>,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: Coordinates {
                x: 0,
                y: 0,
            },
            tail: Coordinates {
                x: 0,
                y: 0,
            },
            tail_positions: HashSet::new(),
        }
    }

    fn adjust(&mut self, adjustment: Adjustment) {
        for i in 0..adjustment.distance {
            // Which way is the tail going to move?
            /*let tail_direction = match self.head.orientation(self.tail) {
                Direction::None => Direction::None, // Tail does not need to move
                Direction::Up => Direction::Up,
                Direction::Down => Direction::Down,

            };*/
            self.head.step(adjustment.direction);

        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
struct Coordinates {
    x: isize,
    y: isize,
}

impl Coordinates {
    fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Down => self.y += 1,
            Direction::Up => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::DownRight => { self.step(Direction::Down); self.step(Direction::Right)},
            Direction::DownLeft => { self.step(Direction::Down); self.step(Direction::Left)},
            Direction::UpRight => { self.step(Direction::Up); self.step(Direction::Right)},
            Direction::UpLeft => { self.step(Direction::Up); self.step(Direction::Left)},
            Direction::None => {},
        };
    }

    /* Returns the direction other would need to move to get closer to self */
    fn orientation(&self, other: Coordinates) -> Direction {
        let x = self.x - other.x;
        let y = self.y - other.y;
        if x == 0 && y == 0 {
            Direction::None 
        } else if x >= 1 {
            if y == 0 {
                Direction::Right
            } else if y > 0 {
                Direction::UpRight
            } else {
                Direction::DownRight
            }
        } else if x < 0 {
            if y == 0 {
                Direction::Left
            } else if y > 0 {
                Direction::UpLeft
            } else {
                Direction::DownLeft
            }
        } else  { // Implied x == 0
            if y < 0 {
                Direction::Down
            } else {
                Direction::Up
            }
        }
    }
}

#[derive(Copy,Clone,PartialEq,Debug)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
}

#[derive(Copy,Clone,PartialEq,Debug)]
struct Adjustment {
    direction: Direction,
    distance: usize,
}

#[test]
fn test_coordinates_orientation()
{
    let middle = Coordinates{x: 0, y: 0};
    let middle2 = Coordinates{x: 0, y: 0};
    assert_eq!(Direction::None, middle.orientation(middle2));
    assert_eq!(Direction::None, middle2.orientation(middle));
    let right = Coordinates{x:1, y: 0};
    assert_eq!(Direction::Left, middle.orientation(right));
    assert_eq!(Direction::Right, right.orientation(middle));
    let left = Coordinates{x:-1, y:0};
    assert_eq!(Direction::Right, middle.orientation(left));
    assert_eq!(Direction::Left, left.orientation(middle));
    let down = Coordinates{x:0, y:-1};
    assert_eq!(Direction::Up, middle.orientation(down));
    assert_eq!(Direction::Down, down.orientation(middle));
    let up = Coordinates{x:0, y:1};
    assert_eq!(Direction::Down, middle.orientation(up));
    assert_eq!(Direction::Up, up.orientation(middle));
    // Diagonals
    assert_eq!(Direction::UpLeft, left.orientation(down));
    assert_eq!(Direction::DownRight, down.orientation(left));
    assert_eq!(Direction::UpRight, right.orientation(down));
    assert_eq!(Direction::DownLeft, down.orientation(right));
    // > 1 distance
    assert_eq!(Direction::Right, right.orientation(left));
    assert_eq!(Direction::Left, left.orientation(right));
    assert_eq!(Direction::Up, up.orientation(down));
    assert_eq!(Direction::Down, down.orientation(up));
    let downleft = Coordinates{x:-1,y:-1};
    let upright = Coordinates{x:1,y:1};
    assert_eq!(Direction::DownLeft, downleft.orientation(upright));
}

fn parse_instructions(input: &str) -> Vec<Coordinates> {
    let coords = Vec::new();
    coords
}

#[test]
fn test_parse_instructions() {
    let test_input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    let test_instructions = vec![
        Coordinates{x: 0, y: -4},
        Coordinates{x: -3, y: 0},
        Coordinates{x: 0, y: 1},
        Coordinates{x: 4, y: 0},
        Coordinates{x: 0, y: 1},
        Coordinates{x: -5, y: 0},
        Coordinates{x: 2, y: 0},
    ];
    assert_eq!(test_instructions, parse_instructions(&test_input));
}

fn main() {
    let buf = fs::read_to_string("2022d8p1.txt").unwrap();
    println!("Directions: {:?}", parse_instructions(&buf));
}