use std::fs;
use std::cmp;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Location(i8, i8);

impl Location {
    fn r#move(mut self, direction: (i8, i8)) { 
        self.0 += direction.0;
        self.1 += direction.1;
    }
}


fn main() {
    let file_path = "day04/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    println!("{:?}", lines);
    
    let rows = lines.len();
    let cols = lines[0].len();
    
    for row in 0..rows {
        let x = rows;
        for col in 0..cols {
            if lines[row][col] == 'X' {
                let x = rows;
                let mut stack = vec!['S', 'A', 'M'];
                match ((row as i64-1).cmp(&0), 
                       (col as i64-1).cmp(&0), 
                       (row+1).cmp(&(rows - 1)), 
                       (col+1).cmp(&(cols - 1))) 
                {
                    // top left
                    (Ordering::Less, Ordering::Less, _, _) => {},
                    // top right
                    (Ordering::Less, _, _, Ordering::Greater) => {},
                    // bottom left
                    (_, Ordering::Less, Ordering::Greater, _) => {},
                    // bottom right
                    (_, _, Ordering::Greater, Ordering::Greater) => {},
                    // top row
                    (Ordering::Less, _, _, _) => {},
                    // bottom row
                    (_, _, Ordering::Greater, _) => {},
                    // leftmost col
                    (_, Ordering::Less, _, _) => {},
                    // rightmost column
                    (_, _, _,Ordering::Greater) => {},
                    // all other
                    _ => {}
                }
                stack.pop();
            }
        }
    }
}
