use cached::proc_macro::cached;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let banks: Vec<i64> = contents
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    return redistribute(banks).0;
}

fn part2(contents: String) -> i64 {
    let banks: Vec<i64> = contents
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    return redistribute(banks).1;
}

#[cached]
fn redistribute(mut banks: Vec<i64>) -> (i64, i64) {
    let mut seen: HashMap<Vec<i64>, i64> = HashMap::new();
    let mut t = 0;

    let bank_len = banks.len();

    while !seen.contains_key(&banks) {
        seen.insert(banks.clone(), t);

        let mut max = 0;
        let mut ix = 0;

        for i in 0..bank_len {
            if banks[i] > max {
                max = banks[i];
                ix = i;
            }
        }

        let blocks = banks[ix];
        banks[ix] = 0;
        let each_gets = blocks / bank_len as i64;
        let remainder = blocks % bank_len as i64;

        for i in 0..bank_len {
            banks[i] += each_gets;
        }

        for i in 0..remainder {
            banks[(ix as i64 + i + 1) as usize % bank_len] += 1;
        }

        t += 1;
    }

    return (t, t - seen.get(&banks).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents = "0\t2\t7\t0".to_string();

        assert_eq!(part1(contents), 5);
    }

    #[test]
    fn p2_test() {
        let contents = "0\t2\t7\t0".to_string();

        assert_eq!(part2(contents), 4);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2017".to_string();
    let day = "6".to_string();

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
        "\nPart 1:\nSteps to see repeat configuration: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nCycle length: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}