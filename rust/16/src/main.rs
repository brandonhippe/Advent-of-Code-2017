use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{VecDeque, HashMap};
use regex::Regex;

fn get_prog_str(prog_order: VecDeque<char>) -> String {
    prog_order.iter().fold("".to_string(), |mut s, c| { s.push(*c); s })
}

fn part1(contents: String, num_programs: i64) -> String {
    let ins_regex = Regex::new(r"(s)(\d+)|(x)(\d+)/(\d+)|(p)(\w+)/(\w+)").unwrap();
    let mut prog_order: VecDeque<char> = VecDeque::from_iter((0..num_programs).map(|c| char::from_u32(c as u32 + 'a' as u32).unwrap()));

    for line in contents.lines() {
        for ins in ins_regex.captures_iter(line) {
            if let Some(_) = ins.get(1) {
                let spin = ins.get(2).unwrap().as_str().parse::<usize>().unwrap();
                prog_order.rotate_right(spin % num_programs as usize);
            }
            if let Some(_) = ins.get(3) {
                let ix1 = ins.get(4).unwrap().as_str().parse::<usize>().unwrap();
                let ix2 = ins.get(5).unwrap().as_str().parse::<usize>().unwrap();
                prog_order.swap(ix1, ix2);
            }
            if let Some(_) = ins.get(6) {
                let p1 = ins.get(7).unwrap().as_str();
                let p2 = ins.get(8).unwrap().as_str();
                let ix1 = prog_order.iter().enumerate().filter(|(_ix, c)| **c == p1.chars().next().unwrap()).map(|(ix, _c)| ix).next().unwrap();
                let ix2 = prog_order.iter().enumerate().filter(|(_ix, c)| **c == p2.chars().next().unwrap()).map(|(ix, _c)| ix).next().unwrap();
                prog_order.swap(ix1, ix2);
            }
        }
    }

    return get_prog_str(prog_order);
}

fn part2(contents: String) -> String {
    let num_programs: i64 = 16;
    let ins_regex = Regex::new(r"(s)(\d+)|(x)(\d+)/(\d+)|(p)(\w+)/(\w+)").unwrap();
    let mut prog_order: VecDeque<char> = VecDeque::from_iter((0..num_programs).map(|c| char::from_u32(c as u32 + 'a' as u32).unwrap()));

    let mut test_num: i64 = 0;
    let mut states: HashMap<String, i64> = HashMap::new();
    let tot_tests: i64 = 1_000_000_000;
    while test_num < tot_tests {
        for line in contents.lines() {
            for ins in ins_regex.captures_iter(line) {
                if let Some(_) = ins.get(1) {
                    let spin = ins.get(2).unwrap().as_str().parse::<usize>().unwrap();
                    prog_order.rotate_right(spin % num_programs as usize);
                }
                if let Some(_) = ins.get(3) {
                    let ix1 = ins.get(4).unwrap().as_str().parse::<usize>().unwrap();
                    let ix2 = ins.get(5).unwrap().as_str().parse::<usize>().unwrap();
                    prog_order.swap(ix1, ix2);
                }
                if let Some(_) = ins.get(6) {
                    let p1 = ins.get(7).unwrap().as_str();
                    let p2 = ins.get(8).unwrap().as_str();
                    let ix1 = prog_order.iter().enumerate().filter(|(_ix, c)| **c == p1.chars().next().unwrap()).map(|(ix, _c)| ix).next().unwrap();
                    let ix2 = prog_order.iter().enumerate().filter(|(_ix, c)| **c == p2.chars().next().unwrap()).map(|(ix, _c)| ix).next().unwrap();
                    prog_order.swap(ix1, ix2);
                }
            }
        }

        let state = get_prog_str(prog_order.clone());
        if states.contains_key(&state) {
            let last_occ = states.get(&state).unwrap();
            let cycle_len = test_num - last_occ;
            test_num = tot_tests - ((tot_tests - test_num) % cycle_len);
        } else {
            states.insert(state, test_num);
        }
        
        prog_order = VecDeque::from_iter(prog_order.into_iter());
        test_num += 1;
    }

    return get_prog_str(prog_order);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, 5), "baedc".to_string());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "16".to_string();

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
        "\nPart 1:\nProgram order: {}\nRan in {:.5?}",
        part1(contents.clone(), 16),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nProgram Order: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}