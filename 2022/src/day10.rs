use crate::common::read_lines;
use std::path::Path;

pub fn day10<P: AsRef<Path>>(file_path: P) -> (i32, i32) {
    let mut signal_strength = 0;
    let mut crt = vec![vec!["."; 40]; 6];

    let cycle_checks = [20, 60, 100, 140, 180, 220];

    let mut x: i32 = 1;
    let mut clock = 0;
    for line in read_lines(file_path) {
        match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["addx", count] => {
                clock += 1;
                do_cycle(cycle_checks, clock, x, &mut signal_strength, &mut crt);

                clock += 1;
                do_cycle(cycle_checks, clock, x, &mut signal_strength, &mut crt);
                x += count.parse::<i32>().unwrap();
            }
            _ => {
                clock += 1;
                do_cycle(cycle_checks, clock, x, &mut signal_strength, &mut crt);
            }
        }
    }

    for line in crt {
        println!("{:?}", line);
    }

    (signal_strength, 0)
}

fn do_cycle(
    cycle_checks: [i32; 6],
    clock: i32,
    x: i32,
    signal_strength: &mut i32,
    crt: &mut Vec<Vec<&str>>,
) {
    if cycle_checks.contains(&clock) {
        *signal_strength += clock * x;
    }

    let crt_pix = clock - 1;
    let crt_y = (((crt_pix / 40) as f64).floor()) as usize;
    let crt_x = (crt_pix as usize - (crt_y * 40)) as i32;

    if x == crt_x || x - 1 == crt_x || x + 1 == crt_x {
        crt[crt_y][crt_x as usize] = "#";
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::day10;

    #[test]
    fn test_case() {
        assert_eq!(day10("src/test_data/day10.txt"), (13140, 0));
    }
}
