/*
--- Day 7: The Treachery of Whales ---

A giant whale has decided your submarine is its next meal, and it's much faster than you are. There's nowhere to run!

Suddenly, a swarm of crabs (each in its own tiny submarine - it's too deep for them otherwise) zooms in to rescue you! They seem to be preparing to blast a hole in the ocean floor; sensors indicate a massive underground cave system just beyond where they're aiming!

The crab submarines all need to be aligned before they'll have enough power to blast a large enough hole for your submarine to get through. However, it doesn't look like they'll be aligned before the whale catches you! Maybe you can help?

There's one major catch - crab submarines can only move horizontally.

You quickly make a list of the horizontal position of each crab (your puzzle input). Crab submarines have limited fuel, so you need to find a way to make all of their horizontal positions match while requiring them to spend as little fuel as possible.

For example, consider the following horizontal positions:

16,1,2,0,4,2,7,1,2,14

This means there's a crab with horizontal position 16, a crab with horizontal position 1, and so on.

Each change of 1 step in horizontal position of a single crab costs 1 fuel. You could choose any horizontal position to align them all on, but the one that costs the least fuel is horizontal position 2:

    Move from 16 to 2: 14 fuel
    Move from 1 to 2: 1 fuel
    Move from 2 to 2: 0 fuel
    Move from 0 to 2: 2 fuel
    Move from 4 to 2: 2 fuel
    Move from 2 to 2: 0 fuel
    Move from 7 to 2: 5 fuel
    Move from 1 to 2: 1 fuel
    Move from 2 to 2: 0 fuel
    Move from 14 to 2: 12 fuel

This costs a total of 37 fuel. This is the cheapest possible outcome; more expensive outcomes include aligning at position 1 (41 fuel), position 3 (39 fuel), or position 10 (71 fuel).

Determine the horizontal position that the crabs can align to using the least fuel possible. How much fuel must they spend to align to that position?

--- Part Two ---

The crabs don't seem interested in your proposed solution. Perhaps you misunderstand crab engineering?

As it turns out, crab submarine engines don't burn fuel at a constant rate. Instead, each change of 1 step in horizontal position costs 1 more unit of fuel than the last: the first step costs 1, the second step costs 2, the third step costs 3, and so on.

As each crab moves, moving further becomes more expensive. This changes the best horizontal position to align them all on; in the example above, this becomes 5:

    Move from 16 to 5: 66 fuel
    Move from 1 to 5: 10 fuel
    Move from 2 to 5: 6 fuel
    Move from 0 to 5: 15 fuel
    Move from 4 to 5: 1 fuel
    Move from 2 to 5: 6 fuel
    Move from 7 to 5: 3 fuel
    Move from 1 to 5: 10 fuel
    Move from 2 to 5: 6 fuel
    Move from 14 to 5: 45 fuel

This costs a total of 168 fuel. This is the new cheapest possible outcome; the old alignment position (2) now costs 206 fuel instead.

Determine the horizontal position that the crabs can align to using the least fuel possible so they can make you an escape route! How much fuel must they spend to align to that position?

*/
#![feature(stdin_forwarders, int_abs_diff)]
use std::io;
use std::collections::HashMap;

fn main() {
    let mut cache = HashMap::new();
    println!("Testing {} to {} = {}", 16, 5, cost_to_move(&mut cache, 16, 5));
    println!("Initial line of crabs:");
    let mut crabline = String::new();
    io::stdin().read_line(&mut crabline).unwrap();
    let crabs: Vec<usize> = Vec::from_iter(
        crabline
            .trim()
            .split(",")
            .map(|sint| sint.parse::<usize>().unwrap()),
    );
    let min_crab = *crabs.iter().min().unwrap();
    let max_crab = *crabs.iter().max().unwrap();
    let unique_crabs: Vec<usize> = (min_crab..max_crab).collect();
    let mut min_fuel: Option<usize> = None;
    let mut min_fuel_pos: Option<usize> = None;
    for unique_crab in unique_crabs.iter() {
        let mut fuel_cost = 0;
        for crab in crabs.iter() {
            fuel_cost += cost_to_move(&mut cache, *crab, *unique_crab);
        }
        if fuel_cost < *min_fuel.get_or_insert(fuel_cost) {
            min_fuel = Some(fuel_cost);
            min_fuel_pos = Some(*unique_crab);
        }
    }
    println!("Best pos is {} with a cost of {}", min_fuel_pos.unwrap(), min_fuel.unwrap());
}

fn cost_to_move(cache: &mut HashMap<isize, usize>, from: usize, to: usize) -> usize {
    let abs_diff: isize = from.abs_diff(to).try_into().unwrap();
    return *cache.entry(abs_diff).or_insert({
        let mut distance = 0;
        for i in 1..abs_diff+1 {
            distance += i;
        }
        distance.try_into().unwrap()
    })
}
