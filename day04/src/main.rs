use std::fs;

#[derive(Clone, Debug, PartialEq)]
struct Direction(i8, i8);

const ALL_DIRECTIONS: &[Direction] = &[
    Direction(-1, 0),  // UP
    Direction(1, 0),   // DOWN
    Direction(0, -1),  // LEFT
    Direction(0, 1),   // RIGHT
    Direction(-1, -1), // UP_LEFT
    Direction(-1, 1),  // UP_RIGHT
    Direction(1, -1),  // DOWN_LEFT
    Direction(1, 1),   // DOWN_RIGHT
];

fn main() {
    let file_path = "day04/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    let mut result = 0;

    let rows = lines.len();
    let cols = lines[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if lines[row][col] == 'X' {
                let allowed = allowed_directions(row, col, rows, cols);

                for direction in allowed {
                    let mut stack: Vec<char> = vec!['S', 'A', 'M'];
                    let mut new_row = row as i64 + direction.0 as i64;
                    let mut new_col = col as i64 + direction.1 as i64;
                    let mut valid = true;
                    
                    while !stack.is_empty()
                        && valid
                    {
                        valid = lines[new_row as usize][new_col as usize] == stack.pop().unwrap();
                        if allowed_directions(new_row as usize, new_col as usize, rows, cols)
                            .contains(&direction) {
                            new_row += direction.0 as i64;
                            new_col += direction.1 as i64;
                        } else {
                            break;
                        }
                    }
                    if stack.is_empty() && valid {
                        result += 1;
                    }
                }
            }
        }
    }
    println!("Part 2: {:?}", result);
}

fn allowed_directions(row: usize, col: usize, rows: usize, cols: usize) -> Vec<Direction> {
    ALL_DIRECTIONS
        .iter()
        .filter(|&&Direction(r_off, c_off)| {
            let new_r = row as isize + r_off as isize;
            let new_c = col as isize + c_off as isize;
            new_r >= 0 && new_r < rows as isize && new_c >= 0 && new_c < cols as isize
        })
        .cloned()
        .collect()
}
