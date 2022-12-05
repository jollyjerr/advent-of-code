mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use crate::{day1::day1, day2::day2, day3::day3, day4::day4, day5::day5};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let file = path_to_data(&args[2]);

    println!("Running day {} with data {}", day, file);

    let day_str = String::as_str(day);

    if day_str == "1" {
        day1(file);
    } else if day_str == "2" {
        day2(file);
    } else if day_str == "3" {
        day3(file);
    } else if day_str == "4" {
        day4(file);
    }  else if day_str == "5" {
        day5(file);
    }
}

fn path_to_data(name: &String) -> String {
    format!("src/data/{}", name)
}
