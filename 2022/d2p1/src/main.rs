/*
--- Day 2: Rock Paper Scissors ---

The Elves begin to set up camp on the beach. To decide whose tent gets to be
closest to the snack storage, a giant Rock Paper Scissors tournament is already
in progress.

Rock Paper Scissors is a game between two players. Each game contains many
rounds; in each round, the players each simultaneously choose one of Rock,
Paper, or Scissors using a hand shape. Then, a winner for that round is
selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats
Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy
guide (your puzzle input) that they say will be sure to help you win. "The
first column is what your opponent is going to play: A for Rock, B for Paper,
and C for Scissors. The second column--" Suddenly, the Elf is called away to
help with someone's tent.

The second column, you reason, must be what you should play in response: X for
Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious,
so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your
total score is the sum of your scores for each round. The score for a single
round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3
for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if
the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you
should calculate the score you would get if you were to follow the strategy
guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z

This strategy guide predicts and recommends the following:

	In the first round, your opponent will choose Rock (A), and you should
choose Paper (Y). This ends in a win for you with a score of 8 (2 because you
chose Paper + 6 because you won).
	In the second round, your opponent will choose Paper (B), and you should
choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
	The third round is a draw with both players choosing Scissors, giving you a
score of 3 + 3 = 6.

In this example, if you were to follow the strategy guide, you would get a
total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your
strategy guide?

*/

use std::fs;

const TEST_INPUT: &str = "A Y
B X
C Z";

#[derive(Clone,Debug,PartialEq)]
enum RPSPlay {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Debug)]
enum RPSResult {
    Win,
    Lose,
    Draw,
}

#[derive(Clone, PartialEq, Debug)]
struct RPSRule {
    play: RPSPlay,
    score: usize,
    winner: RPSPlay,
}

impl RPSRule {
    fn result(self: &Self, other: &RPSPlay) -> RPSResult {
        if self.winner == *other {
            return RPSResult::Lose
        }
        if self.play == *other {
            return RPSResult::Draw
        }
        return RPSResult::Win
    }

    fn score(self: &Self, other: &RPSPlay) -> usize {
        return match self.result(other) {
            RPSResult::Lose => 0,
            RPSResult::Win => 6,
            RPSResult::Draw => 3,
        } + self.score;
    }
}

static ROCK: RPSRule = RPSRule {
    play: RPSPlay::Rock,
    score: 1,
    winner: RPSPlay::Paper
};
static PAPER: RPSRule = RPSRule {
    play: RPSPlay::Paper,
    score: 2,
    winner: RPSPlay::Scissors
};
static SCISSORS: RPSRule = RPSRule {
    play: RPSPlay::Scissors,
    score: 3,
    winner: RPSPlay::Rock,
};

fn parse_strategy_guide(input: &str) -> Vec<(&'static RPSRule, &'static RPSRule)> {
    let mut parsed = Vec::new();
    let lines = input.lines();
    lines.for_each(|line| {
        let mut s = line.split(" ");
        let their_play = match s.next().expect(&format!("Could not parse {}", line)) {
            "A" => &ROCK,
            "B" => &PAPER,
            "C" => &SCISSORS,
            _ => panic!("Invalid left option: {}", line),
        };
        let my_play = match s.next().expect(&format!("Could not parse {}", line)) {
            "X" => &ROCK,
            "Y" => &PAPER,
            "Z" => &SCISSORS, 
            _ => panic!("Invalid right option: {}", line),
        };
        parsed.push((their_play, my_play));
    });
    parsed
}

fn score_guide(guide: Vec<(&'static RPSRule, &'static RPSRule)>) -> usize {
    guide.iter().map(|(theirs, mine)| mine.score(&theirs.play)).sum()
}

#[test]
fn test_parse_strategy_guide() {
    let test_guide: Vec<(&'static RPSRule, &'static RPSRule)> = vec![
        (&ROCK, &PAPER),
        (&PAPER, &ROCK),
        (&SCISSORS, &SCISSORS),
    ];
    let parsed_guide = parse_strategy_guide(TEST_INPUT);
    assert_eq!(test_guide, parsed_guide);
    let (first_theirs, first_mine) = parsed_guide[0];
    assert_eq!(first_mine.result(&first_theirs.play), RPSResult::Win);
    assert_eq!(first_mine.score, 2);
    assert_eq!(first_mine.score(&first_theirs.play), 8);
    let (second_theirs, second_mind) = parsed_guide[1];
    assert_eq!(second_mind.result(&second_theirs.play), RPSResult::Lose);
    assert_eq!(second_mind.score(&second_theirs.play), 1);
    assert_eq!(score_guide(parsed_guide), 15)
}

fn main() {
    let buf = fs::read_to_string("2022d2p1.txt").unwrap();
    println!("The total score for this guide played perfectly would be: {}", score_guide(parse_strategy_guide(&buf)));
}
