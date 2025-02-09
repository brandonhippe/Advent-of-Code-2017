use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> String {
    let mut programs: Vec<Program> = Vec::new();
    let mut all_programs: HashSet<String> = HashSet::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts[0].to_string();
        let weight = parts[1]
            .trim_matches(|c| c == '(' || c == ')')
            .parse()
            .unwrap();
        let children: Vec<String> = if parts.len() > 2 {
            parts[3..]
                .iter()
                .map(|s| s.trim_matches(',').to_string())
                .collect()
        } else {
            Vec::new()
        };
        programs.push(Program::new(weight, children));
        all_programs.insert(name);
    }

    for program in programs.iter() {
        for child in program.children.iter() {
            all_programs.remove(child);
        }
    }


    return all_programs.iter().next().unwrap().to_string();
}

fn part2(contents: String) -> i64 {
    let mut prog_names: HashSet<String> = HashSet::new();
    let mut all_programs: HashMap<String, Program> = HashMap::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts[0].to_string();
        let weight = parts[1]
            .trim_matches(|c| c == '(' || c == ')')
            .parse()
            .unwrap();
        let children: Vec<String> = if parts.len() > 2 {
            parts[3..]
                .iter()
                .map(|s| s.trim_matches(',').to_string())
                .collect()
        } else {
            Vec::new()
        };
        all_programs.insert(name.clone(), Program::new(weight, children));
        prog_names.insert(name);
    }

    for program in all_programs.values() {
        for child in program.children.iter() {
            prog_names.remove(child);
        }
    }

    let bottom_prog = prog_names.iter().next().unwrap().to_string();
    let mut curr_prog: Program = all_programs.get_mut(&bottom_prog).unwrap().clone();
    let mut diff = 0;

    while !curr_prog.clone().is_balanced(&mut all_programs) {
        let mut weights: Vec<i64> = Vec::new();
        for child in curr_prog.children.iter() {
            let child_program = all_programs.get(child).unwrap().clone();
            weights.push(child_program.total_weight(&mut all_programs));
        }

        let unbalanced_ix = weights
            .iter()
            .position(|&w| weights.iter().filter(|&x| *x == w).count() == 1)
            .unwrap();
        diff = weights[unbalanced_ix] - weights[(unbalanced_ix + 1) % weights.len()];

        curr_prog = all_programs
            .get(&curr_prog.children[unbalanced_ix])
            .unwrap()
            .clone();
    }

    return curr_prog.weight - diff;
}

#[derive(Debug, Clone)]
struct Program {
    weight: i64,
    children: Vec<String>,
}

impl Program {
    fn new(weight: i64, children: Vec<String>) -> Program {
        Program { weight, children }
    }

    fn total_weight(&self, programs: &mut HashMap<String, Program>) -> i64 {
        let mut total = self.weight;
        for child in self.children.iter() {
            let child_program = programs.get(child).unwrap().clone();
            total += child_program.total_weight(programs);
        }

        return total;
    }

    fn is_balanced(self, programs: &mut HashMap<String, Program>) -> bool {
        if self.children.len() == 0 {
            return true;
        }

        let mut weights: Vec<i64> = Vec::new();
        for child in self.children.iter() {
            let child_program = programs.get(child).unwrap().clone();
            weights.push(child_program.total_weight(programs));
        }

        return weights.iter().all(|&w| w == weights[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), "tknk".to_string());
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 60);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2017".to_string();
    let day = "7".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("rust_{}_{}", year, day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };


    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nBottom program: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nCorrect weight of incorrect program: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}