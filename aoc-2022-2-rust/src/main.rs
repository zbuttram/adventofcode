use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const ROCK: &str = "ROCK";
const PAPER: &str = "PAPER";
const SCISSORS: &str = "SCISSORS";

fn main() {
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

        let throwScore = match player {
            ROCK => 1,
            PAPER => 2,
            SCISSORS => 3,
            _ => 0
        };

        score += throwScore;

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

fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader.lines();
}