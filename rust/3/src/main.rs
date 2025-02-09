use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let num = contents.trim().parse::<i64>().unwrap();
    return num_pos(num).0.abs() + num_pos(num).1.abs();
}

fn part2(contents: String) -> i64 {
    let num = contents.trim().parse::<i64>().unwrap();
    let mut area: HashMap<(i64, i64), i64> = HashMap::new();
    area.insert((0, 0), 1);

    let mut ix = 2;
    let mut last = 1;

    while last < num {
        let pos = num_pos(ix);
        let mut sum = 0;
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                if let Some(val) = area.get(&(pos.0 + i, pos.1 + j)) {
                    sum += val;
                }
            }
        }

        area.insert(pos, sum);
        last = sum;

        ix += 1;
    }

    return last;
}

fn num_pos(n: i64) -> (i64, i64) {
    let layer = (n as f64).sqrt().ceil() as i64 / 2;
    let side = layer * 2;

    let mut pos = (layer, -layer);
    let mut remaining = n - (2 * layer - 1).pow(2);
    let mut ix = 1;
    let mut mult = 1;
    while remaining > side {
        pos = (
            pos.0 + (side * mult * (ix == 0) as i64),
            pos.1 + (side * mult * (ix == 1) as i64),
        );
        remaining -= side;
        ix = (ix + 1) % 2;
        if ix == 0 {
            mult *= -1;
        }
    }

    pos = (
        pos.0 + (remaining * mult * (ix == 0) as i64),
        pos.1 + (remaining * mult * (ix == 1) as i64),
    );

    return pos;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let mut contents = "1".to_string();
        assert_eq!(part1(contents), 0);
        contents = "12".to_string();
        assert_eq!(part1(contents), 3);
        contents = "23".to_string();
        assert_eq!(part1(contents), 2);
        contents = "1024".to_string();
        assert_eq!(part1(contents), 31);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2017".to_string();
    let day = "3".to_string();

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
        "\nPart 1:\nSteps to origin: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nFirst number written larger than input: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}