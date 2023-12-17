type Point = (usize, usize);

fn day10(lines: Vec<String>, is_part_two: bool) -> usize {
    let mut start: Point = (0, 0);

    'locate_start: for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = (i, j);
                break 'locate_start;
            }
        }
    }

    let graph: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let mut steps = 0;
    let mut points: Vec<Point> = vec![start];

    while points.len() > 0 {
        let mut round_results: Vec<Point> = vec![];

        for point in points {
            round_results.append(&mut walk(&graph, point));
        }

        if round_results.len() > 0 {
            steps += 1;
        }

        points = round_results;
    }

    steps
}

fn walk(lines: &Vec<Vec<char>>, point: Point) -> Vec<Point> {
    let mut out = vec![];

    let up = (point.0 - 1, point.1);
    let left = (point.0, point.1 - 1);
    let down = (point.0 + 1, point.1);
    let right = (point.0, point.1 + 1);

    // match lines.get(up.0).and_then(|l| l.get(up.1)).unwrap_or('.') {
    //     '|',
    //     '7',
    //     'F',
    //     _other => {
    //         // noop
    //     }
    // }

    out
}

#[cfg(test)]
mod tests {
    use crate::common::read_lines;
    use crate::day10::day10;

    #[test]
    fn test_one() {
        assert_eq!(day10(read_lines("src/data/day10a.txt"), false), 8);
    }

    // #[test]
    // fn test_two() {
    //     assert_eq!(day10(read_lines("src/data/day10b.txt"), false), 8);
    // }
}

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
