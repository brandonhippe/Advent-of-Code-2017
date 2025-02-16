use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::VecDeque;

fn knot_hash(cards: Vec<i64>, lengths: Vec<i64>, rounds: i64) -> Vec<i64> {
    let mut curr_pos: usize = 0;
    let mut skip: usize = 0;
    let mut cards = cards;

    for _ in 0..rounds {
        for len in lengths.iter() {
            cards.rotate_left(curr_pos);

            let amt: usize = *len as usize;
            let mut reversed: Vec<i64> = cards.iter().take(amt).cloned().collect();
            reversed.reverse();

            cards.drain(0..amt);
            cards.extend(reversed);
            cards.rotate_right(amt);
            cards.rotate_right(curr_pos);
            curr_pos = (curr_pos + amt + skip) % cards.len();
            skip += 1;
        }
    }

    return cards;
}

fn part1(contents: String, start_len: i64) -> i64 {
    let lengths = {
        let line = contents.lines().next().unwrap();
        let mut lengths: Vec<i64> = vec![];
        for c in line.split(",") {
            lengths.push(c.trim().parse::<i64>().unwrap());
        }
        lengths
    };
    
    let final_cards = knot_hash((0..start_len).collect(), lengths, 1);
    return final_cards[0] * final_cards[1];
}

fn part2(contents: String, start_len: i64) -> String {
    let mut lengths = {
        let line = contents.lines().next().unwrap();
        let mut lengths: Vec<i64> = vec![];
        for c in line.chars() {
            lengths.push(c as i64);
        }
        lengths
    };
    lengths.extend(vec![17, 31, 73, 47, 23]);
    
    let final_cards = knot_hash((0..start_len).collect(), lengths, 64);
    let mut dense_hash: Vec<i64> = vec![];
    for i in 0..16 {
        let mut block: i64 = 0;
        for j in 0..16 {
            block ^= final_cards[i * 16 + j];
        }
        dense_hash.push(block);
    }

    return dense_hash
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, 5), 12);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents, 256), "33efeb34ea91902bb2f59c9920caa6cd".to_string());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "10".to_string();

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
        "\nPart 1:\nProduct of first two numbers: {}\nRan in {:.5?}",
        part1(contents.clone(), 256),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nKnot Hash: {}\nRan in {:.5?}",
        part2(contents.clone(), 256),
        part2_timer.elapsed()
    );
}