use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::VecDeque;

fn part1(contents: String) -> i64 {
    let step_amt = contents.trim().parse::<usize>().unwrap() + 1;
    let mut buff: VecDeque<i64> = VecDeque::from(vec![0]);

    for insert in 1..=2017 {
        buff.rotate_left(step_amt % buff.len());
        buff.push_front(insert)
    }

    return *buff.iter().skip(1).next().unwrap();
}

fn part2(contents: String) -> i64 {
    let step_amt = contents.trim().parse::<i64>().unwrap() + 1;
    let mut zero_ix: i64 = 0;
    let mut after_zero: Option<i64> = None;
    
    for insert in 1..=50_000_000 {
        zero_ix = (((zero_ix - step_amt + 1) % insert) + insert) % insert;
        if zero_ix == insert - 1 {
            after_zero = Some(insert);
        }
    }
    
    return after_zero.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        assert_eq!(part1("3".to_string()), 638);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 0);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "17".to_string();

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
        "\nPart 1:\nValue after 2017: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nValue after 0: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}