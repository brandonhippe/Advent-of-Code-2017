use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashSet;

fn part1(contents: String) -> i64 {
    return contents.lines().filter(|line| {
        let words: HashSet<&str> = HashSet::from_iter(line.split_whitespace());
        words.len() == line.split_whitespace().count()
    }).count() as i64
}

fn part2(contents: String) -> i64 {
    return contents.lines().filter(|line| {
        for (i, word) in line.split_whitespace().enumerate() {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            for (j, other_word) in line.split_whitespace().enumerate() {
                if i != j {
                    let mut other_chars: Vec<char> = other_word.chars().collect();
                    other_chars.sort();
                    if chars == other_chars {
                        return false;
                    }
                }
            }
        }

        return true;
    }).count() as i64
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "4".to_string();

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
        "\nPart 1:\nValid passphrases: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nValid passphrases: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}