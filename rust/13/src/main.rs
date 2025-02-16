use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut score: i64 = 0;
    for line in contents.lines() {
        let mut parts = line.split(": ");
        let depth: i64 = parts.next().unwrap().parse().unwrap();
        let range: i64 = parts.next().unwrap().parse().unwrap();

        if depth % (2 * (range - 1)) == 0 {
            score += depth * range;
        }
    }

    return score;
}

fn part2(contents: String) -> i64 {
    let mut scanners: Vec<(i64, i64)> = Vec::new();
    for line in contents.lines() {
        let mut parts = line.split(": ");
        scanners.push((
            parts.next().unwrap().parse::<i64>().unwrap(),
            parts.next().unwrap().parse::<i64>().unwrap(),
        ));
    }

    let mut start: i64 = 1;
    let mut caught: bool = true;
    while caught {
        caught = false;
        for (depth, range) in &scanners {
            if (start + depth) % (2 * (range - 1)) == 0 {
                caught = true;
                break;
            }
        }
        if !caught {
            break;
        }
        start += 1;
    }

    return start;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 24);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 10);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "13".to_string();

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
        "\nPart 1:\nTrip severity: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nDelay: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}