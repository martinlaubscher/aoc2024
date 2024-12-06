use regex::Regex;
use std::fs;

fn main() {
    let file_path = "day03/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mul_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let multiplications: Vec<&str> = mul_re.find_iter(&contents).map(|m| m.as_str()).collect();
    println!("Part 1: {}", sumproduct(&multiplications));

    // match the don't - do blocks
    let dont_re = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    // match everything after the final don't if there is no more do
    let dont_end_re = Regex::new(r"don't\(\).*").unwrap();
    // replace newline chars so matching works, then remove all the unwanted parts using regex
    let contents_tmp: String = dont_re
        .replace_all(&contents.replace("\n", ""), "")
        .parse()
        .unwrap();
    let contents_cleaned = dont_end_re.replace_all(&contents_tmp, "");
    // extract multiplications from cleaned contents and multiply/sum as asked
    let multiplications_cleaned: Vec<&str> = mul_re
        .find_iter(&contents_cleaned)
        .map(|m| m.as_str())
        .collect();
    println!("Part 2: {}", sumproduct(&multiplications_cleaned));
}

fn sumproduct(multiplications: &[&str]) -> i32 {
    let num_re = Regex::new(r"(\d{1,3})").unwrap();
    multiplications.iter().fold(0, |acc, &multiplication| {
        let nums = num_re
            .find_iter(multiplication)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        acc + (nums[0] * nums[1])
    })
}
