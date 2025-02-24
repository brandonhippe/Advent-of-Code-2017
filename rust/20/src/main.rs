use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::iter::zip;
use regex::Regex;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct PointVector {
    x: i64,
    y: i64,
    z: i64,
}

impl PointVector {
    fn new(v_str: &str) -> PointVector {
        let int_re = Regex::new(r"-?\d+").unwrap();
        let ints: Vec<i64> = int_re.find_iter(v_str).map(|v| v.as_str().parse::<i64>().unwrap()).collect();
        if ints.len() != 3 {
            panic!("Points must have 3 coordinates");
        }
        
        PointVector {
            x: ints[0],
            y: ints[1],
            z: ints[2]
        }
    }
    
    fn update(&mut self, other: &PointVector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

fn closest_point(particles: &Vec<PointVector>) -> i64 {
    particles.iter().enumerate().min_by_key(|d| {
        d.1.x.abs() + d.1.y.abs() + d.1.z.abs()
    }).unwrap().0 as i64
}

fn part1(contents: String) -> i64 {
    let mut particles: Vec<PointVector> = Vec::new();
    let mut velocities: Vec<PointVector> = Vec::new();
    let mut accelerations: Vec<PointVector> = Vec::new();
    for line in contents.lines() {
        let particle_data: Vec<&str> = line.split(", ").collect();
        particles.push(PointVector::new(particle_data[0]));
        velocities.push(PointVector::new(particle_data[1]));
        accelerations.push(PointVector::new(particle_data[2]));
    }
    
    let mut update_count: i64 = 1;
    let mut closest: i64;
    
    loop {
        closest = closest_point(&particles);
        for (v, a) in zip(velocities.iter_mut(), accelerations.iter_mut()) {
            v.update(a);
        }
        for (p, v) in zip(particles.iter_mut(), velocities.iter_mut()) {
            p.update(v);
        }
        update_count += 1;
        
        if update_count % 1000 == 0 && closest == closest_point(&particles) {
            break;
        }
    }
    
    return closest_point(&particles);
}

fn part2(contents: String) -> i64 {
    let mut particles: Vec<PointVector> = Vec::new();
    let mut velocities: Vec<PointVector> = Vec::new();
    let mut accelerations: Vec<PointVector> = Vec::new();
    for line in contents.lines() {
        let particle_data: Vec<&str> = line.split(", ").collect();
        particles.push(PointVector::new(particle_data[0]));
        velocities.push(PointVector::new(particle_data[1]));
        accelerations.push(PointVector::new(particle_data[2]));
    }
    
    let mut update_count: i64 = 1;

    loop {
        for (v, a) in zip(velocities.iter_mut(), accelerations.iter_mut()) {
            v.update(a);
        }
        
        let mut particle_counts: HashMap<PointVector, i64> = HashMap::new();
        for (p, v) in zip(particles.iter_mut(), velocities.iter_mut()) {
            p.update(v);
            let p_key = p.clone();
            *particle_counts.entry(p_key).or_insert(0) += 1;
        }

        let remove_ix: Vec<usize> = (0..particles.len()).filter_map(|i| {
            let p = &particles[i];
            if *particle_counts.get(p).unwrap() >= 2 {
                Some(i)
            } else {
                None
            }
        }).collect();

        for ix in remove_ix.iter().rev() {
            particles.remove(*ix);
            velocities.remove(*ix);
            accelerations.remove(*ix);
        }
        
        update_count += 1;
        if update_count % 1000 == 0 && remove_ix.len() == 0 {
            break;
        }

    }
    
    return particles.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 0);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2017".to_string();
    let day = "20".to_string();

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
        "\nPart 1:\nClosest particle: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nRemaining particles: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}