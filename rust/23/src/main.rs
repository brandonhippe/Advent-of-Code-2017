use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use regex::Regex;

#[derive(Clone, Debug)]
struct Program<'a> {
    instructions: Vec<&'a str>,
    pc: i64,
    registers: HashMap<char, i64>,
    mul_times: i64
}

impl Default for Program<'_> {
    fn default() -> Program<'static> {
        Program {
            instructions: vec![],
            pc: 0,
            registers: HashMap::new(),
            mul_times: 0,
        }
    }
}

impl Program<'_> {
    fn run_p1(&mut self) -> i64 {
        while 0 <= self.pc && (self.pc as usize) < self.instructions.len() {
            self.execute_instruction();
        }
        
        return self.mul_times;
    }

    fn run_p2(&mut self) -> i64 {
        while 0 <= self.pc && (self.pc as usize) < 8_usize.min(self.instructions.len()) {
            self.execute_instruction();
        }

        let b: i64 = *self.registers.get(&'b').unwrap();
        let c: i64 = *self.registers.get(&'c').unwrap();
        let step_size: usize = Regex::new(r"\d+").unwrap().find(self.instructions[self.instructions.len() - 2]).unwrap().as_str().parse::<usize>().unwrap();
        let mut primes = sieve_of_eratosthenes(c);

        return (b..(c+1)).step_by(step_size).filter(|n| !*primes.entry(*n).or_insert(true)).count() as i64;
    }
    
    fn execute_instruction(&mut self) {
        let ins = self.instructions[self.pc as usize];
        let ins_words: Vec<&str> = ins.split_whitespace().collect();
        
        let reg: char = ins_words[1].chars().next().unwrap();
        let num: i64 = if let Ok(v) = ins_words[ins_words.len() - 1].parse::<i64>() {
            v
        } else {
            *self.registers.get(&ins_words[ins_words.len() - 1].chars().next().unwrap()).unwrap_or(&0)
        };
        
        match ins.split_whitespace().next().unwrap() {
            "set" => {
                self.registers.insert(reg, num);
                self.pc += 1;
            },
            "sub" => {
                *self.registers.entry(reg).or_insert(0) -= num;
                self.pc += 1;
            },
            "mul" => {
                *self.registers.entry(reg).or_insert(0) *= num;
                self.pc += 1;
                self.mul_times += 1;
            },
            "jnz" => {
                let check = if let Ok(v) = ins_words[1].parse::<i64>() {
                    v
                } else {
                    *self.registers.get(&reg).unwrap_or(&0)
                };
    
                if check != 0 {
                    self.pc += num;
                } else {
                    self.pc += 1;
                }
            },
            _ => panic!("Unknown instruction!")
        }
    }
}

fn sieve_of_eratosthenes(n: i64) -> HashMap<i64, bool> {
    let mut primes: HashMap<i64, bool> = HashMap::from([(1, false)]);
    let mut p: i64 = 2;
    while (p * p) <= n {
        if *primes.entry(p).or_insert(true) {
            for i in ((p*p)..(n+1)).step_by(p as usize) {
                primes.insert(i, false);
            }
        }
        p += 1;
    }

    return primes;
}

fn part1(contents: String) -> i64 {
    let mut program = Program {
        instructions: contents.lines().map(|l| l.trim()).collect(),
        ..Default::default()
    };
    
    return program.run_p1();
}

fn part2(contents: String) -> i64 {
    let mut program = Program {
        instructions: contents.lines().map(|l| l.trim()).collect(),
        registers: HashMap::from([('a', 1)]),
        ..Default::default()
    };

    return program.run_p2();
}

#[cfg(test)]
mod tests {
    use super::*;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "23".to_string();

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
        "\nPart 1:\nMul instruction invoked: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nValue in register h: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}