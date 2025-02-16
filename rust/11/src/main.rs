use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn part1(contents: String) -> i64 {
    let line = contents.lines().next().unwrap();
    let moves = HashMap::from([
        ("n", (0, -1)),
        ("s", (0, 1)),
        ("ne", (1, -1)),
        ("nw", (-1, 0)),
        ("se", (1, 0)),
        ("sw", (-1, 1)),
    ]);

    let mut position: (i64, i64) = (0, 0);
    for direction in line.split(",") {
        if let Some(&move_delta) = moves.get(direction) {
            position.0 += move_delta.0;
            position.1 += move_delta.1;
        } else {
            println!("Unknown direction: {}", direction);
        }
    }
    let (x, y) = position;
    let z: i64 = -x - y;

    return (x.abs() + y.abs() + z.abs()) / 2;
}

fn part2(contents: String) -> i64 {
    let line = contents.lines().next().unwrap();
    let moves = HashMap::from([
        ("n", (0, -1)),
        ("s", (0, 1)),
        ("ne", (1, -1)),
        ("nw", (-1, 0)),
        ("se", (1, 0)),
        ("sw", (-1, 1)),
    ]);

    let mut position: (i64, i64) = (0, 0);
    let mut max_distance = 0;
    for direction in line.split(",") {
        if let Some(&move_delta) = moves.get(direction) {
            position.0 += move_delta.0;
            position.1 += move_delta.1;
        } else {
            println!("Unknown direction: {}", direction);
        }

        let (x, y) = position;
        let z: i64 = -x - y;
        max_distance = max_distance.max((x.abs() + y.abs() + z.abs()) / 2);
    }

    return max_distance;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 0);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 0);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "11".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("{}", day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };

    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nDistance away: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMax Distance: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}