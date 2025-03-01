use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::VecDeque;

fn part1(contents: String) -> i64 {
    let components: Vec<(i64, i64)> = Vec::from_iter(contents.lines().map(|l| {
        let mut split = l.split("/");
        (split.next().unwrap().parse::<i64>().unwrap(), split.next().unwrap().parse::<i64>().unwrap())
    }));
    
    let mut bridges: VecDeque<Vec<(i64, i64)>> = VecDeque::from_iter(components.iter().filter_map(|c| {
        if c.0 == 0 || c.1 == 0 {
            Some(vec![*c])
        } else {
            None
        }
    }));
    
    let mut max_strength: i64 = 0;
    while let Some(bridge) = bridges.pop_front() {
        let strength = bridge.iter().map(|c| c.0 + c.1).sum::<i64>();
        max_strength = max_strength.max(strength);

        let last = bridge.iter().last().unwrap();
        for add_c in components.iter().filter(|c| {
            !bridge.contains(c) && !bridge.contains(&(c.1, c.0))
        }) {
            if add_c.0 == last.1 {
                let mut new_bridge = bridge.clone();
                new_bridge.push(*add_c);
                bridges.push_back(new_bridge);
            } else if add_c.1 == last.1 {
                let mut new_bridge = bridge.clone();
                new_bridge.push((add_c.1, add_c.0));
                bridges.push_back(new_bridge);
            }
        }
    }

    return max_strength;
}

fn part2(contents: String) -> i64 {
    let components: Vec<(i64, i64)> = Vec::from_iter(contents.lines().map(|l| {
        let mut split = l.split("/");
        (split.next().unwrap().parse::<i64>().unwrap(), split.next().unwrap().parse::<i64>().unwrap())
    }));
    
    let mut bridges: VecDeque<Vec<(i64, i64)>> = VecDeque::from_iter(components.iter().filter_map(|c| {
        if c.0 == 0 || c.1 == 0 {
            Some(vec![*c])
        } else {
            None
        }
    }));
    
    let mut max_length: usize = 0;
    let mut max_strength: i64 = 0;
    while let Some(bridge) = bridges.pop_front() {
        let strength = bridge.iter().map(|c| c.0 + c.1).sum::<i64>();
        if bridge.len() >= max_length {
            if bridge.len() > max_length {
                max_strength = 0;
            }
            max_strength = max_strength.max(strength);
        }

        let last = bridge.iter().last().unwrap();
        for add_c in components.iter().filter(|c| {
            !bridge.contains(c) && !bridge.contains(&(c.1, c.0))
        }) {
            if add_c.0 == last.1 {
                let mut new_bridge = bridge.clone();
                new_bridge.push(*add_c);
                bridges.push_back(new_bridge);
            } else if add_c.1 == last.1 {
                let mut new_bridge = bridge.clone();
                new_bridge.push((add_c.1, add_c.0));
                bridges.push_back(new_bridge);
            }
        }
    }

    return max_strength;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 31);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 19);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "24".to_string();

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
        "\nPart 1:\nStrongest: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nStrongest Longest: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}