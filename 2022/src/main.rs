use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let file = File::open("src/data.txt").expect("file not found");
    let buffer = BufReader::new(file);

    let lines: Vec<String> = buffer.lines().map(|l| l.expect("no line found")).collect();

    let mut counters: Vec<i32> = Vec::new();
    let mut counter: i32 = 0;
    for line in lines {
        if line == "" {
            counters.push(counter);
            counter = 0
        } else {
            let new_int: i32 = line.parse().unwrap();
            counter += new_int
        }
    }

    counters.sort_by(|a, b| b.cmp(a));

    println!("One: {}", counters[0]);
    println!("Two: {}", counters[1]);
    println!("Three: {}", counters[2]);
    println!("Total: {}", counters[0] + counters[1] + counters[2]);
}
