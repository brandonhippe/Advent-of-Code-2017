use cached::proc_macro::cached;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    return run_instructions(
        contents
            .lines()
            .map(|l| Instruction::from_string(l))
            .collect(),
    )
    .0;
}

fn part2(contents: String) -> i64 {
    return run_instructions(
        contents
            .lines()
            .map(|l| Instruction::from_string(l))
            .collect(),
    )
    .1;
}

#[cached]
fn run_instructions(instructions: Vec<Instruction>) -> (i64, i64) {
    let mut registers: HashMap<String, i64> = HashMap::new();
    let mut max = 0;

    for instr in instructions {
        let test_register = registers.entry(instr.test_register).or_insert(0);
        let test_amt = instr.test_amt;

        let test_op = instr.test_op;

        let test = match test_op.as_str() {
            "==" => *test_register == test_amt,
            "!=" => *test_register != test_amt,
            ">" => *test_register > test_amt,
            "<" => *test_register < test_amt,
            ">=" => *test_register >= test_amt,
            "<=" => *test_register <= test_amt,
            _ => panic!("Invalid test operation"),
        };

        if test {
            let register = registers.entry(instr.register).or_insert(0);
            *register += instr.amt;
            max = max.max(*register);
        }
    }

    let max_register = registers.values().max().unwrap();
    return (*max_register, max);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    register: String,
    amt: i64,
    test_register: String,
    test_op: String,
    test_amt: i64,
}

impl Instruction {
    fn from_string(s: &str) -> Instruction {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let register = parts[0].to_string();
        let multiplier = if parts[1] == "inc" { 1 } else { -1 };
        let amt = parts[2].parse::<i64>().unwrap() * multiplier;
        let test_register = parts[4].to_string();
        let test_op = parts[5].to_string();
        let test_amt = parts[6].parse::<i64>().unwrap();

        Instruction {
            register,
            amt,
            test_register,
            test_op,
            test_amt,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 1);
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
    let day = "8".to_string();

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
        "\nPart 1:\nLargest value in any register at end of program: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLargest value in any register at any point: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}