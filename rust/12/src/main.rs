use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_group(
    connections: &HashMap<i64, Vec<i64>>,
    start: i64,
) -> HashSet<i64> {
    let mut queue: Vec<i64> = vec![start];
    let mut visited: HashSet<i64> = HashSet::new();

    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        visited.insert(current);
        if let Some(neighbors) = connections.get(&current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    queue.push(*neighbor);
                }
            }
        }
    }

    return visited;
}

fn part1(contents: String) -> i64 {
    let mut connections: HashMap<i64, Vec<i64>> = HashMap::new();
    for line in contents.lines() {
        let mut parts = line.split(" <-> ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        let program: i64 = left.parse().unwrap();
        for connection in right.split(",") {
            let connection: i64 = connection.trim().parse().unwrap();
            connections
                .entry(program)
                .or_insert_with(Vec::new)
                .push(connection);
        }
    }

    return get_group(&connections, 0).len() as i64;
}

fn part2(contents: String) -> i64 {
    let mut connections: HashMap<i64, Vec<i64>> = HashMap::new();
    for line in contents.lines() {
        let mut parts = line.split(" <-> ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        let program: i64 = left.parse().unwrap();
        for connection in right.split(",") {
            let connection: i64 = connection.trim().parse().unwrap();
            connections
                .entry(program)
                .or_insert_with(Vec::new)
                .push(connection);
        }
    }

    let mut groups: i64 = 0;
    while !connections.is_empty() {
        let start = connections.keys().next().unwrap();
        let group = get_group(&connections, *start);
        for program in group.iter() {
            connections.remove(program);
        }
        groups += 1;
    }

    return groups;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 6);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 2);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "12".to_string();

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
        "\nPart 1:\nSize of group with program 0: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nNumber of groups: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}