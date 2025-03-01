use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Tape {
    state: String,
    pos: i64,
    contents: HashMap<i64, usize>,
    instructions: HashMap<String, Vec<(usize, i64, String)>>
}

impl Default for Tape {
    fn default() -> Tape {
        Tape {
            state: "".to_string(),
            pos: 0,
            contents: HashMap::new(),
            instructions: HashMap::new(),
        }
    }
}

impl Tape {
    fn new(state: String, contents: String) -> Tape {
        let mut instructions: HashMap<String, Vec<(usize, i64, String)>> = HashMap::new();
        for ins_str in contents.split("\n\n").skip(1) {
            let mut ins_split = ins_str.lines();
            let mut state: String = ins_split.next().unwrap().split_whitespace().last().unwrap().to_string();
            state.remove(state.len() - 1);
            
            let mut tasks: Vec<(usize, i64, String)> = Vec::new();
            for _ in 0..2 {
                ins_split.next();
                let new_val: usize = Regex::new(r"\d+").unwrap().find(ins_split.next().unwrap()).unwrap().as_str().parse::<usize>().unwrap();
                let move_dir: i64 = if ins_split.next().unwrap().contains("right") {
                    1
                } else {
                    -1
                };
                let mut next_state: String = ins_split.next().unwrap().split_whitespace().last().unwrap().to_string();
                next_state.remove(next_state.len() - 1);

                tasks.push((new_val, move_dir, next_state));
            }

            instructions.insert(state, tasks);
        }

        Tape {
            state: state,
            instructions: instructions,
            ..Default::default()
        }
    }

    fn step(&mut self) {
        let tape_val = self.contents.entry(self.pos).or_insert(0);
        let (new_val, move_dir, next_state) = &self.instructions.get(&self.state).unwrap()[*tape_val];
        *tape_val = *new_val;
        self.pos += move_dir;
        self.state = next_state.clone();
    }
}

fn part1(contents: String) -> i64 {
    let mut initial_state = contents.split("\n\n").next().unwrap().lines();
    let mut state: String = initial_state.next().unwrap().split_whitespace().last().unwrap().to_string();
    state.remove(state.len() - 1);

    let mut tape: Tape = Tape::new(state, contents.clone());
    for _ in 0..Regex::new(r"\d+").unwrap().find(initial_state.next().unwrap()).unwrap().as_str().parse::<i64>().unwrap() {
        tape.step();
    }

    return tape.contents.values().filter(|v| **v == 1).count() as i64;
}

fn part2(contents: String) -> String {
    return "Christmas has been saved!".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 3);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "25".to_string();

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
        "\nPart 1:\nChecksum: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n{}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}