use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};

fn follow_path(contents: String) -> (String, i64) {

    let mut turns: HashMap<(i64, i64), HashMap<(i64, i64), (i64, i64)>> = HashMap::new();
    let mut letter_pos: HashMap<(i64, i64), char> = HashMap::new();
    let mut available: HashSet<(i64, i64)> = HashSet::new();
    let mut start: Option<(i64, i64)> = None;
    
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                ' ' => (),
                _ => {
                    available.insert((x as i64, y as i64));
                    if y == 0 {
                        start = Some((x as i64, y as i64));
                    }
                }
            }
            match c {
                '|' => (),
                '+' => { turns.insert((x as i64, y as i64), HashMap::new()); },
                '-' => (),
                ' ' => (),
                _ => { letter_pos.insert((x as i64, y as i64), c); }
            }
        }
    }
    
    for (pos, v) in turns.iter_mut() {
        let exist_dirs: Vec<(i64, i64)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)].iter().filter(|off| available.contains(&(pos.0 - off.0, pos.1 - off.1))).map(|off| *off).collect();
        if exist_dirs.len() != 2 {
            panic!("Unsure what to do when available neighbors to turn != 2");
        }
        
        v.insert(exist_dirs[0], (exist_dirs[1].0 * -1, exist_dirs[1].1 * -1));
        v.insert(exist_dirs[1], (exist_dirs[0].0 * -1, exist_dirs[0].1 * -1));
    }
    
    let mut pos = start.unwrap();
    let mut direction = (0, 1);
    let mut order_string = String::new();
    let mut total_steps: i64 = 0;
    
    while available.contains(&pos) {
        if turns.contains_key(&pos) {
            direction = *turns[&pos].get(&direction).unwrap();
        }
        if letter_pos.contains_key(&pos) {
            order_string.push(*letter_pos.get(&pos).unwrap());
        }
        
        pos = (pos.0 + direction.0, pos.1 + direction.1);
        total_steps += 1;
    }
    
    return (order_string, total_steps);
}
fn part1(contents: String) -> String {
    return follow_path(contents).0;
}

fn part2(contents: String) -> i64 {
    return follow_path(contents).1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), "ABCDEF".to_string());
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 38);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "19".to_string();

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
        "\nPart 1:\nPath: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nPath length: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}