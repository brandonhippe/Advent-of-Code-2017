use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap};

fn part1(contents: String) -> i64 {
    let mut map: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((x as i64, y as i64));
            }
        }
    }

    let mut node: (i64, i64) = (contents.lines().count() as i64 / 2, contents.lines().count() as i64 / 2);
    let mut direction: (i64, i64) = (0, -1);
    
    let mut infections: i64 = 0;
    for _ in 0..10000 {
        if map.contains(&node) {
            direction = (-direction.1, direction.0);
            map.remove(&node);
        } else {
            direction = (direction.1, -direction.0);
            map.insert(node);
            infections += 1;
        }
        node = (node.0 + direction.0, node.1 + direction.1);
    }
    
    return infections;
}

fn part2(contents: String) -> i64 {
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((x as i64, y as i64), 2);
            }
        }
    }
    
    let mut node: (i64, i64) = (contents.lines().count() as i64 / 2, contents.lines().count() as i64 / 2);
    let mut direction: (i64, i64) = (0, -1);
    
    let mut infections: i64 = 0;
    for _ in 0..10000000 {
        let state = map.entry(node).or_insert(0);
        direction = match state {
            0 => (direction.1, -direction.0),
            1 => direction,
            2 => (-direction.1, direction.0),
            3 => (-direction.0, -direction.1),
            _ => panic!("Unknown node state"),
        };
        
        *state += 1;
        *state %= 4;
        infections += (*state == 2) as i64;
        node = (node.0 + direction.0, node.1 + direction.1);
    }
    
    return infections;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn p1_test() {
        let contents =
        fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 5587);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 2511944);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "22".to_string();

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
        "\nPart 1:\nInfections: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nInfections: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}