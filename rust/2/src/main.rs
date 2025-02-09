use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    return contents
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let max = nums.iter().max().unwrap();
            let min = nums.iter().min().unwrap();
            max - min
        })
        .sum::<i64>();
}

fn part2(contents: String) -> i64 {
    return contents
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            for i in 0..nums.len() - 1 {
                for j in i + 1..nums.len() {
                    if nums[i] % nums[j] == 0 {
                        return nums[i] / nums[j];
                    } else if nums[j] % nums[i] == 0 {
                        return nums[j] / nums[i];

                    }
                }
            }

            0
        })
        .sum::<i64>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents = "5 1 9 5\n7 5 3\n2 4 6 8\n".to_string();

        assert_eq!(part1(contents), 18);
    }

    #[test]
    fn p2_test() {
        let contents = "5 9 2 8\n9 4 7 3\n3 8 6 5\n".to_string();

        assert_eq!(part2(contents), 9);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2017".to_string();
    let day = "2".to_string();

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
        "\nPart 2:\nSum of rows: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}