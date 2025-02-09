use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut vals: Vec<i64> = contents.lines().map(|line| line.parse().unwrap()).collect();
    let mut steps = 0;
    let mut i = 0;

    while i < vals.len() {
        let jump = vals[i];
        vals[i] += 1;
        i = (i as i64 + jump) as usize;
        steps += 1;
    }

    return steps as i64;
}

fn part2(contents: String) -> i64 {
    let mut vals: Vec<i64> = contents.lines().map(|line| line.parse().unwrap()).collect();
    let mut steps = 0;
    let mut i = 0;

    while i < vals.len() {
        let jump = vals[i];
        if jump >= 3 {
            vals[i] -= 1;
        } else {
            vals[i] += 1;
        }
        i = (i as i64 + jump) as usize;
        steps += 1;
    }


    return steps as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 5);
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
    let day = "5".to_string();

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
        "\nPart 1:\nSteps to reach exit: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSteps to reach exit: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}