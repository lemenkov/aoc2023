use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        let mut sum = 0;
        for raw_line in lines {
            if let Ok(line) = raw_line {
                let vec = parse_digits(line.as_str());
                println!("LINE: {} {}", vec[0]*10 + vec[ vec.len() - 1],  line);
                sum += vec[0]*10 + vec[ vec.len() - 1]
            }
        }
        println!("SUM: {}", sum);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_digits(t_num: &str) -> Vec<u32> {
    t_num
        .chars()
        .filter_map(|a| a.to_digit(10))
        .collect()
}
