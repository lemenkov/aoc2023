use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output\
    let rgb0= (12, 13, 14);
    if let Ok(lines) = read_lines("./input") {
        let mut sum = 0u16;
        for raw_line in lines {
            if let Ok(line) = raw_line {
                let (id, rgb1) = parse_line(line);
                if compare_rgb(rgb0, rgb1) {
                    println!("LINE {}", id);
                    sum += id
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

fn parse_line(line: String) -> (u16, (u8, u8, u8)) {
    let v: Vec<&str> = line.split(&[':', ' ', ',', ';'][..]).filter(|&x| !x.is_empty()).collect();
    let (mut r, mut g, mut b) = (0u8, 0u8, 0u8);
    let id = v[1].parse::<u16>().unwrap();
    let iter = v[2..].into_iter().step_by(2).zip(v[2..].into_iter().skip(1).step_by(2)).collect::<Vec<_>>();
    for (number, color) in iter {
        let n = number.parse::<u8>().unwrap();
        match color {
            &"red" => r = if n > r { n } else { r } ,
            &"green" => g = if n > g { n } else { g },
            &"blue" => b = if n > b { n } else { b },
            _ => println!("UNKNOWN: {}", color),
        }
    }
    (id, (r, g, b))
}

fn compare_rgb ((red0, green0, blue0): (u8, u8, u8), (red1, green1, blue1): (u8, u8, u8)) -> bool {
    if red0 >= red1 && green0 >= green1 && blue0 >= blue1 {
        true
    }
    else {
        false
    }
}
