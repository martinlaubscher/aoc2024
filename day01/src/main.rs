use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "day01/list.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut left_col = contents
        .lines()
        .map(|line| {
            line.split_ascii_whitespace().collect::<Vec<&str>>()[0]
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>();

    let mut right_col = contents
        .lines()
        .map(|line| {
            line.split_ascii_whitespace().collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>();

    left_col.sort();
    right_col.sort();

    let part_1 = left_col
        .iter()
        .zip(right_col.iter())
        .fold(0, |acc, (&a, &b)| acc + (a - b).abs());

    let mut freq: HashMap<i32, i32> = Default::default();
    right_col.iter().for_each(|&a| {
        freq.entry(a).and_modify(|e| *e += 1).or_insert(1);
    });

    let part_2 = left_col
        .iter()
        .fold(0, |acc, &a| acc + (a * freq.get(&a).unwrap_or(&0)));

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
