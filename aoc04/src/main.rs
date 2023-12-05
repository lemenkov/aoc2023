use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        let mut sum = 0u32;

        let mut cards: Vec<(u32, u32)> = vec![];

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

                cards.push((count, 1));
            }
        }
        println!("SUM: {}", sum);
        println!("CARDS TOTAL: {}" , count_cards(cards));

    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_cards(mut cards: Vec<(u32, u32)>) -> u32 {
    let lenght = cards.len();
    let mut total = 0u32;

    let mut i: usize = 0;
    while i < lenght {
        let mut j: usize = 0;
        while j < cards[i].0.try_into().unwrap() {
            cards[i + j + 1].1 += cards[i].1;
            j += 1;
        }
        total += cards[i].1;
        i += 1;
    }

    return total;
}
