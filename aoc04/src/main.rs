use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        let mut sum = 0u32;
        for raw_line in lines {
            if let Ok(line) = raw_line {
                let v: Vec<&str> = line.split(&[':', '|'][..]).filter(|&x| !x.is_empty()).collect();
                // let card_num: Vec<&str> = v[0].split(&[' '][..]).collect();
                let winning_nums: Vec<&str> = v[1].split(&[' '][..]).filter(|&x| !x.is_empty()).collect();
                let nums: Vec<&str> = v[2].split(&[' '][..]).filter(|&x| !x.is_empty()).collect();

                let mut count = 0u32;
                let base: u32 = 2;
                for num in winning_nums {
                    if nums.contains(&num) {
                        count += 1;
                    };
                };

                if count > 0 {
                    sum += base.pow(count-1);
                }
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
