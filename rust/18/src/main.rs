use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
struct Program<'a> {
    instructions: Vec<&'a str>,
    pc: i64,
    registers: HashMap<char, i64>,
    recv_buf: VecDeque<i64>,
    state: i64,
    send_count: i64,
}

impl Default for Program<'_> {
    fn default() -> Program<'static> {
        Program {
            instructions: vec![],
            pc: 0,
            registers: HashMap::new(),
            recv_buf: VecDeque::new(),
            state: 0,
            send_count: 0,
        }
    }
}

impl Program<'_> {
    fn run(&mut self, p2: bool) -> Option<i64> {
        if self.state == 2 {
            return None;
        }

        while 0 <= self.pc && (self.pc as usize) < self.instructions.len() {
            let ins = self.instructions[self.pc as usize];
            let ins_words: Vec<&str> = ins.split_whitespace().collect();
            
            let reg: char = ins_words[1].chars().next().unwrap();
            let num: i64 = if let Ok(v) = ins_words[ins_words.len() - 1].parse::<i64>() {
                v
            } else {
                *self.registers.get(&ins_words[ins_words.len() - 1].chars().next().unwrap()).unwrap_or(&0)
            };
            
            match ins.split_whitespace().next().unwrap() {
                "snd" => {
                    self.pc += 1;
                    self.send_count += 1;
                    return Some(num);
                },
                "set" => {
                    self.registers.insert(reg, num);
                    self.pc += 1;
                },
                "add" => {
                    *self.registers.entry(reg).or_insert(0) += num;
                    self.pc += 1;
                },
                "mul" => {
                    *self.registers.entry(reg).or_insert(0) *= num;
                    self.pc += 1;
                },
                "mod" => {
                    *self.registers.entry(reg).or_insert(0) %= num;
                    self.pc += 1;
                },
                "rcv" => {
                    if p2 || num != 0 {
                        if let Some(recv) = self.recv_buf.pop_front() {
                            self.state = 0;
                            self.registers.insert(reg, recv);
                        } else {
                            if self.state != 2 {
                                self.state += 1;
                            }
                            return None;
                        }
                    }
                    self.pc += 1;
                },
                "jgz" => {
                    let check = if let Ok(v) = ins_words[1].parse::<i64>() {
                        v
                    } else {
                        *self.registers.get(&reg).unwrap_or(&0)
                    };

                    if check > 0 {
                        self.pc += num;
                    } else {
                        self.pc += 1;
                    }
                },
                _ => panic!("Unknown instruction!")
            }
        }

        self.state = 2;
        return None;
    }
}

fn part1(contents: String) -> i64 {
    let mut program = Program {
        instructions: contents.lines().map(|l| l.trim()).collect(),
        ..Default::default()
    };

    let mut last_recv = -1;
    while let Some(recv) = program.run(false) {
        last_recv = recv;
    }
    return last_recv;
}

fn part2(contents: String) -> i64 {
    let instructions: Vec<&str> = contents.lines().map(|l| l.trim()).filter(|l| l.len() > 0).collect();
    let mut p0 = Program {
        instructions: instructions.clone(),
        registers: HashMap::from([('p', 0)]),
        ..Default::default()
    };
    let mut p1 = Program {
        instructions: instructions.clone(),
        registers: HashMap::from([('p', 1)]),
        ..Default::default()
    };

    let mut running: &mut Program<'_> = &mut p1;
    let mut other: &mut Program<'_> = &mut p0;

    while other.state != 2 && running.state != 2 {
        let temp: &mut Program<'_> = other;
        other = running;
        running = temp;
        
        while let Some(send_val) = running.run(true) {
            other.recv_buf.push_back(send_val);
        }
    }

    return p1.send_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 4);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 3);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "18".to_string();

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
        "\nPart 1:\nRecovered frequency: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nValues sent: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}