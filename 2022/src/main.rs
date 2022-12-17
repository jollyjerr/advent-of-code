mod common;
mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::env;

#[derive(Debug)]
enum AOCResult {
    Numbers((i32, i32)),
    BigNums((i64, i64)),
    Strings((String, String)),
    PointerSized((usize, usize)),
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = String::as_str(&args[1]);
    let file = path_to_data(&args[2]);

    let result = match day {
        "1" => AOCResult::Numbers(day1::day1(file)),
        "2" => AOCResult::Numbers(day2::day2(file)),
        "3" => AOCResult::Numbers(day3::day3(file)),
        "4" => AOCResult::Numbers(day4::day4(file)),
        "5" => AOCResult::Strings(day5::day5(file)),
        "6" => AOCResult::PointerSized(day6::day6(file)),
        "7" => AOCResult::Numbers(day7::day7(file)),
        "8" => AOCResult::Numbers(day8::day8(file)),
        "9" => AOCResult::Numbers(day9::day9(file)),
        "10" => AOCResult::Numbers(day10::day10(file)),
        "11" => AOCResult::BigNums(day11::day11(file)),
        _ => panic!(),
    };

    println!("Day {} results (Part 1, Part 2): {:?}", day, result);
}

fn path_to_data(name: &String) -> String {
    format!("src/data/{}", name)
}
