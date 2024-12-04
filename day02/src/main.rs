use std::fs;

fn main() {
    let file_path = "day02/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part_1 = contents.lines().fold(0, |acc, line| {
        let nums = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if (nums.iter().is_sorted() || nums.iter().rev().is_sorted())
            && nums
                .windows(2)
                .all(|w| (w[0] - w[1]).abs() <= 3 && w[0] != w[1])
        {
            acc + 1
        } else {
            acc
        }
    });
    println!("Part 1: {}", part_1);

    let part_2 = contents.lines().fold(0, |acc, line| {
        let nums = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let ascending = nums.windows(2).filter(|&w| w[1] > w[0]).count()
            >= nums.windows(2).filter(|&w| w[0] > w[1]).count();

        let mut danger = false;

        let mut i: usize = 1;
        let mut j: usize = 0;

        while i < nums.len() && j < nums.len() - 1 {
            if nums[i] == nums[j]
                || (nums[i] > nums[j] && !ascending)
                || (nums[i] < nums[j] && ascending)
                || (nums[i] - nums[j]).abs() > 3
            {
                // already had one wrong element and no chance it's the first element
                if danger && j > 0 {
                    return acc;
                // first element might be "wrong", so advance second pointer
                } else if danger {
                    j += 1;
                // first "wrong" element encountered, advance first pointer to check if following elements are correct
                } else {
                    danger = true;
                    i += 1;
                }
            // check next window
            } else {
                j = i;
                i += 1;
            }
        }
        acc + 1
    });
    println!("Part 2: {}", part_2);
}
