use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashSet;

fn dense_hash(line: &str, start_len: i64) -> String {
    let mut lengths = {
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
        .map(|x| format!("{:08b}", x))
        .collect::<String>();
}

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

fn part1(contents: String) -> i64 {
    let key = contents.lines().next().unwrap();

    let mut on_count: i64 = 0;
    for ix in 0..128 {
        let line = format!("{}-{}", key, ix);
        let hash = dense_hash(&line, 256);
        on_count += hash.chars().filter(|&c| c == '1').count() as i64;
    }

    return on_count;
}

fn part2(contents: String) -> i64 {
    let key = contents.lines().next().unwrap();

    let mut on: HashSet<(i64, i64)> = HashSet::new();
    for ix in 0..128 {
        let line = format!("{}-{}", key, ix);
        let hash = dense_hash(&line, 256);
        for (iy, c) in hash.chars().enumerate() {
            if c == '1' {
                on.insert((ix, iy as i64));
            }
        }
    }

    let mut regions: i64 = 0;
    while on.len() > 0 {
        let mut to_visit: Vec<(i64, i64)> = vec![];
        let next = on.iter().next().unwrap().clone();
        to_visit.push(next);
        on.remove(&next);

        while to_visit.len() > 0 {
            let curr = to_visit.pop().unwrap();
            let neighbors = vec![
                (curr.0 - 1, curr.1),
                (curr.0 + 1, curr.1),
                (curr.0, curr.1 - 1),
                (curr.0, curr.1 + 1),
            ];
            for n in neighbors {
                if on.contains(&n) {
                    to_visit.push(n);
                    on.remove(&n);
                }
            }
        }
        regions += 1;
    }

    return regions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 8108);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 1242);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "14".to_string();

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
        "\nPart 1:\nSquares used: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nRegions: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}