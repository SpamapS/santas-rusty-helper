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
            self.head.step(adjustment.direction);
            self.tail.step(self.head.pull(self.tail));
            self.tail_positions.insert(self.tail);
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    fn pull(&self, other: Coordinates) -> Direction {
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
fn test_coordinates_pull()
{
    let middle = Coordinates{x: 0, y: 0};
    let middle2 = Coordinates{x: 0, y: 0};
    assert_eq!(Direction::None, middle.pull(middle2));
    assert_eq!(Direction::None, middle2.pull(middle));
    let right = Coordinates{x:1, y: 0};
    assert_eq!(Direction::Left, middle.pull(right));
    assert_eq!(Direction::Right, right.pull(middle));
    let left = Coordinates{x:-1, y:0};
    assert_eq!(Direction::Right, middle.pull(left));
    assert_eq!(Direction::Left, left.pull(middle));
    let down = Coordinates{x:0, y:-1};
    assert_eq!(Direction::Up, middle.pull(down));
    assert_eq!(Direction::Down, down.pull(middle));
    let up = Coordinates{x:0, y:1};
    assert_eq!(Direction::Down, middle.pull(up));
    assert_eq!(Direction::Up, up.pull(middle));
    // Diagonals
    assert_eq!(Direction::UpLeft, left.pull(down));
    assert_eq!(Direction::DownRight, down.pull(left));
    assert_eq!(Direction::UpRight, right.pull(down));
    assert_eq!(Direction::DownLeft, down.pull(right));
    // > 1 distance
    assert_eq!(Direction::Right, right.pull(left));
    assert_eq!(Direction::Left, left.pull(right));
    assert_eq!(Direction::Up, up.pull(down));
    assert_eq!(Direction::Down, down.pull(up));
    let downleft = Coordinates{x:-1,y:-1};
    let upright = Coordinates{x:1,y:1};
    assert_eq!(Direction::DownLeft, downleft.pull(upright));
    let head = Coordinates{x: 2, y: 0};
    let tail = Coordinates{x: 0, y: 0};
    assert_eq!(Direction::Right, head.pull(tail));
    /*
    3T--
    2-*-
    1-H-
    0123
    */
    assert_eq!(Direction::DownRight, head.pull(Coordinates{x:1, y: 3}));
     /*
    3--T
    2-*-
    1-H-
    0123
    */
    assert_eq!(Direction::DownLeft, head.pull(Coordinates{x:3, y:3}));
    let head = Coordinates{x:3, y:3};
    assert_eq!(Direction::UpRight, head.pull(Coordinates{x:2, y:1}));
    let head = Coordinates{x:1, y:3};
     /*
    3H--
    2*--
    1-T-
    0123
    */
    assert_eq!(Direction::UpLeft, head.pull(Coordinates{x:2, y:1}));
}

fn parse_instructions(input: &str) -> Vec<Adjustment> {
    let mut coords = Vec::new();
    for (dir, distance) in input.lines().map(|line| line.split_once(' ').expect("No space in line")) {
        coords.push(Adjustment{
            direction: match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Could not parse a direction!"),
            },
            distance: distance.parse().expect("Couldn't parse digit")
        });
    }
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
        Adjustment{direction: Direction::Right, distance: 4},
        Adjustment{direction: Direction::Up, distance: 4},
        Adjustment{direction: Direction::Left, distance: 3},
        Adjustment{direction: Direction::Down, distance: 1},
        Adjustment{direction: Direction::Right, distance: 4},
        Adjustment{direction: Direction::Down, distance: 1},
        Adjustment{direction: Direction::Left, distance: 5},
        Adjustment{direction: Direction::Right, distance: 2},
    ];
    let parsed_instructions = parse_instructions(&test_input);
    assert_eq!(test_instructions, parsed_instructions);
    let mut rope = Rope::new();
    parsed_instructions.iter().for_each(|adjustment| rope.adjust(*adjustment));
    assert_eq!(13, rope.tail_positions.len());
    
}

fn main() {
    let buf = fs::read_to_string("2022d8p1.txt").unwrap();
    println!("Directions: {:?}", parse_instructions(&buf));
}