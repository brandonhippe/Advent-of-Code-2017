use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn flatten_block(block: &Vec<String>, skip_x: usize, take: usize) -> String {
    let mut block_str = String::new();
    for r in block {
        for c in r.chars().skip(skip_x).take(take) {
            block_str.push(c);
        }
        block_str.push('/');
    }
    
    block_str.pop();
    return block_str;
}

fn blocken_flat(flat: &str) -> Vec<String> {
    flat.split('/').map(|l| l.to_string()).collect::<Vec<String>>()
}

fn on_pixels(contents: String, iterations: i64) -> i64 {
    let mut image: Vec<String> = vec![
        ".#.".to_string(),
        "..#".to_string(),
        "###".to_string(),
    ];
    let mut mapping: HashMap<String, String> = HashMap::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" => ").collect();
        let mut key_block = blocken_flat(parts[0]);
        let value = parts[1];
        for _ in 0..4 {
            // Insert standard and flipped
            mapping.insert(flatten_block(&key_block, 0, key_block.len()), value.to_string());
            mapping.insert(flatten_block(&key_block.clone().iter().rev().map(|l| l.to_string()).collect::<Vec<String>>(), 0, key_block.len()), value.to_string());

            // Rotate
            key_block = (0..key_block.len())
                .map(|i| key_block.iter().rev().map(|row| row.chars().nth(i).unwrap()).collect())
                .collect();
        }
    }

    for _ in 0..iterations {
        let mut new_image: Vec<String> = vec![];
        let group_dim = if image.len() % 2 == 0 { 2 } else { 3 };

        for y_c in (0..image.len()).step_by(group_dim) {
            let mut image_rows: Vec<String> = vec![String::new(); group_dim + 1];
            let rows: Vec<String> = image.iter().skip(y_c).map(|l| l.to_string()).take(group_dim).collect();

            for x_c in (0..image.len()).step_by(group_dim) {
                for (ix, line) in mapping.get(&flatten_block(&rows, x_c, group_dim)).unwrap().split('/').enumerate() {
                    image_rows[ix].push_str(line);
                }
            }

            for line in image_rows.drain(..) {
                new_image.push(line);
            }
        }

        image = new_image;
    }

    return image.iter().map(|l| l.chars().filter(|c| *c == '#').count() as i64).sum::<i64>();
}

fn part1(contents: String, iterations: i64) -> i64 {
    return on_pixels(contents, iterations);
}

fn part2(contents: String) -> i64 {
    return on_pixels(contents, 18);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, 2), 12);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "21".to_string();

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
        "\nPart 1:\nPixels on: {}\nRan in {:.5?}",
        part1(contents.clone(), 5),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nPixels on: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}