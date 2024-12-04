use regex::Regex;
use std::fs;

fn main() {
    let file_path = "day03/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mul_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let num_re = Regex::new(r"(\d{1,3})").unwrap();
    let multiplications: Vec<&str> = mul_re.find_iter(&contents).map(|m| m.as_str()).collect();

    let part_1 = multiplications.iter().fold(0, |acc, &multiplication| {
        let nums = num_re
            .find_iter(multiplication)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        acc + (nums[0] * nums[1])
    });

    println!("Part 1: {part_1}");
}
