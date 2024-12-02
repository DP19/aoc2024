use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();
    if let Ok(lines) = read_lines("/Users/david.pait/Documents/adventOfCode/2024/day1/inputs/day1.txt") {
        for line in lines.flatten() {
            let parts = line.split("   ").collect::<Vec<_>>();
            let left_int = parts[0].parse::<i32>().unwrap();
            let right_int = parts[1].parse::<i32>().unwrap();
            left_vec.push(left_int);
            right_vec.push(right_int);
        }
    }
    left_vec.sort();
    right_vec.sort();

    part1(left_vec.clone(), right_vec.clone());
    part2(left_vec.clone(), right_vec.clone());

}

fn part1(left_vec: Vec<i32>, right_vec: Vec<i32>) {
    let mut total = 0;
    for n in 0..left_vec.len() {
        if left_vec[n] > right_vec[n] {
            total += left_vec[n] - right_vec[n]
        } else {
            total += right_vec[n] - left_vec[n];
        }
    }

    println!("{:?}", total);
}

fn part2(left_vec: Vec<i32>, right_vec: Vec<i32>) {
    let mut total = 0;
    for i in left_vec {
        let times_in_list = right_vec.iter().filter(|&n| *n == i).count();
        total += i * times_in_list as i32;
    }
    println!("{:?}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
