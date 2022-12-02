mod common;
mod day1;

use crate::day1::day1;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let file =  &args[2];

    println!("Running day {} with data {}", day, file);
    
    let day_str = String::as_str(day);

    if day_str == "1" {
        day1(file);
    } else {
        println!("Day {} does not exist!", day);
    }
}
