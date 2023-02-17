use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    // create a vector of integers filled with 3 0s
    let mut top3: Vec<i32> = vec![0, 0, 0];
    let mut count = 0;

    for line in read_lines("input.txt") {
        let value = line.unwrap();
        if value == "" {
            let min = top3.iter().min().unwrap();
            if count > *min {
                let index = top3.iter().position(|&x| x == *min).unwrap();
                top3[index] = count;
            }
            count = 0;
        } else {
            count += value.parse::<i32>().unwrap();
        }
    }

    let total = top3.iter().sum::<i32>();
    println!("{}", total);
}

fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader.lines();
}