use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut a_val: i64 = contents.lines().next().unwrap().split_whitespace().last().unwrap().parse::<i64>().unwrap();
    let mut b_val: i64 = contents.lines().skip(1).next().unwrap().split_whitespace().last().unwrap().parse::<i64>().unwrap();

    let mut match_count: i64 = 0;
    for _ in 0..40_000_000 {
        a_val *= 16807;
        a_val %= 2147483647;
        
        b_val *= 48271;
        b_val %= 2147483647;

        match_count += ((a_val & 0xffff) == (b_val & 0xffff)) as i64;
    }

    return match_count;
}

fn part2(contents: String) -> i64 {
    let mut a_val: i64 = contents.lines().next().unwrap().split_whitespace().last().unwrap().parse::<i64>().unwrap();
    let mut b_val: i64 = contents.lines().skip(1).next().unwrap().split_whitespace().last().unwrap().parse::<i64>().unwrap();
    
    let mut match_count: i64 = 0;
    for _ in 0..5_000_000 {
        loop {
            a_val *= 16807;
            a_val %= 2147483647;
            if (a_val & 0x3) == 0 {
                break;
            }
        }
        
        loop {
            b_val *= 48271;
            b_val %= 2147483647;
            if (b_val & 0x7) == 0 {
                break;
            }
        }

        match_count += ((a_val & 0xffff) == (b_val & 0xffff)) as i64;
    }

    return match_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, ), 588);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 309);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "15".to_string();

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
        "\nPart 1:\nMatching pairs: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMatching pairs: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}