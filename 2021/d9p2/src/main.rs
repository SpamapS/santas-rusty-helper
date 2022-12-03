/*
--- Day 9: Smoke Basin ---

These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal vents release smoke into the caves that slowly settles like rain.

If you can model how the smoke flows through the caves, you might be able to avoid it and be that much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your puzzle input).

Smoke flows to the lowest point of the area it's in. For example, consider the following heightmap:

2199943210
3987894921
9856789892
8767896789
9899965678

Each number corresponds to the height of a particular location, where 9 is the highest and 0 is the lowest a location can be.

Your first goal is to find the low points - the locations that are lower than any of its adjacent locations. Most locations have four adjacent locations (up, down, left, and right); locations on the edge or corner of the map have three or two adjacent locations, respectively. (Diagonal locations do not count as adjacent.)

In the above example, there are four low points, all highlighted: two are in the first row (a 1 and a 0), one is in the third row (a 5), and one is in the bottom row (also a 5). All other locations on the heightmap have some lower adjacent location, and so are not low points.

The risk level of a low point is 1 plus its height. In the above example, the risk levels of the low points are 2, 1, 6, and 6. The sum of the risk levels of all low points in the heightmap is therefore 15.

Find all of the low points on your heightmap. What is the sum of the risk levels of all low points on your heightmap?

--- Part Two ---

Next, you need to find the largest basins so you know what areas are most important to avoid.

A basin is all locations that eventually flow downward to a single low point. Therefore, every low point has a basin, although some basins are very small. Locations of height 9 do not count as being in any basin, and all other locations will always be part of exactly one basin.

The size of a basin is the number of locations within the basin, including the low point. The example above has four basins.

The top-left basin, size 3:

2199943210
3987894921
9856789892
8767896789
9899965678

The top-right basin, size 9:

2199943210
3987894921
9856789892
8767896789
9899965678

The middle basin, size 14:

2199943210
3987894921
9856789892
8767896789
9899965678

The bottom-right basin, size 9:

2199943210
3987894921
9856789892
8767896789
9899965678

Find the three largest basins and multiply their sizes together. In the above example, this is 9 * 14 * 9 = 1134.

What do you get if you multiply together the sizes of the three largest basins?

*/
#![feature(stdin_forwarders, int_abs_diff)]
use std::io;

type Grid = Vec<Vec<u8>>;

// Returns 10, always higher, for any non-existent position to make the logic easier
fn get_grid_pos(grid: &Grid, x: isize, y: isize) -> u8 {
    if x < 0 || y < 0 {
        return 10;
    }
    match grid.get(y as usize) {
        None => 10,
        Some(row) => match row.get(x as usize) {
            None => 10,
            Some(reading) => *reading,
        },
    }
}

fn main() {
    let mut grid: Grid = Vec::new();
    println!("Input map:");
    for inputline in io::stdin().lines() {
        let gridline: Vec<u8> = inputline
            .unwrap()
            .chars()
            .map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
            .collect();
        grid.push(gridline);
    }
    println!("The grid is {} wide and {} long", grid[0].len(), grid.len());
    let mut total_risk: i64 = 0;
    // For reading in grid position
    for (y, row) in grid.iter().enumerate() {
        for (x, reading) in row.iter().enumerate() {
            let y = isize::try_from(y).unwrap();
            let x = isize::try_from(x).unwrap();
            if get_grid_pos(&grid, x, y - 1) > *reading // up
               && get_grid_pos(&grid, x, y + 1) > *reading // down
               && get_grid_pos(&grid, x - 1, y) > *reading // left
               && get_grid_pos(&grid, x + 1, y) > *reading
            {
                // right
                total_risk += 1 + *reading as i64
            }
        }
    }
    println!("Total Risk: {}", total_risk);
}
