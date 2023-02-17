/* Advent of Code 2022 -- Co-authored by Github Copilot */

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const ROCK: &str = "ROCK";
const PAPER: &str = "PAPER";
const SCISSORS: &str = "SCISSORS";
const WIN: &str = "WIN";
const LOSE: &str = "LOSE";
const TIE: &str = "TIE";

fn main() {
    part_one();
    part_two();
}

fn part_two() {
    let mut score = 0;

    for line in read_lines("input.txt") {
        let text = line.unwrap();

        let throws = text.split(" ").map(|x| {
            match x {
                "A" => ROCK,
                "B" => PAPER,
                "C" => SCISSORS,
                "X" => LOSE,
                "Y" => TIE,
                "Z" => WIN,
                _ => ""
            }
        }).collect::<Vec<&str>>();

        let elf = throws[0];
        let desiredOutcome = throws[1];

        let player = match desiredOutcome {
            WIN => match elf {
                ROCK => PAPER,
                PAPER => SCISSORS,
                SCISSORS => ROCK,
                _ => ""
            },
            LOSE => match elf {
                ROCK => SCISSORS,
                PAPER => ROCK,
                SCISSORS => PAPER,
                _ => ""
            },
            TIE => match elf {
                ROCK => ROCK,
                PAPER => PAPER,
                SCISSORS => SCISSORS,
                _ => ""
            },
            _ => ""
        };

        println!("Player throws {}", player);
        score += throw_score(player);
    }

    println!("SCORE: {}", score);
}

fn part_one() {
    let map: HashMap<&str, &str> = HashMap::from([
        (ROCK, PAPER),
        (PAPER, SCISSORS),
        (SCISSORS, ROCK),
    ]);

    let mut score = 0;

    for line in read_lines("input.txt") {
        let text = line.unwrap();

        let throws = text.split(" ").map(|x| {
            match x {
                "A" | "X" => ROCK,
                "B" | "Y" => PAPER,
                "C" | "Z" => SCISSORS,
                _ => ""
            }
        }).collect::<Vec<&str>>();

        let elf = throws[0];
        let player = throws[1];

        score += throw_score(player);

        if map.get(elf).unwrap() == &player {
            println!("Player wins");
            score += 6;
        } else if map.get(player).unwrap() == &elf {
            println!("Elf wins");
        } else {
            println!("Tie");
            score += 3;
        }
    }

    println!("SCORE: {}", score);
}

fn throw_score(throw: &str) -> i32 {
    match throw {
        ROCK => 1,
        PAPER => 2,
        SCISSORS => 3,
        _ => 0
    }
}

fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader.lines();
}