use std::io;

fn get_line_checksum(nums: Vec<i32>) -> i32 {
    let max_num = nums.iter().fold(0, |acc, &x| std::cmp::max(acc, x));
    let min_num = nums.iter().fold(max_num, |acc, &x| std::cmp::min(acc, x));

    max_num - min_num
}

fn get_line_division(nums: Vec<i32>) -> i32 {
    for (i, a) in nums.iter().enumerate() {
        // only iterate through numbers after the current one, gives a bit more efficiency
        // (but we have to check both ways inside here)
        for b in nums.iter().skip(i + 1) {
            if a % b == 0 {
                return a / b
            }
            if b % a == 0 {
                return b / a
            }
        }
    }
    0
}

struct LineReader;

impl Iterator for LineReader {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut val = String::new();
        match io::stdin().read_line(&mut val) {
            Err(_) => None,
            Ok(_) => {
                let trimmed_val = val.trim();

                if trimmed_val.len() > 1 {
                    Some(trimmed_val.to_owned())
                } else {
                    None
                }
            }
        }
    }
}

fn line_reader() -> LineReader {
    LineReader
}

fn main() {
    println!("Please enter the spreadsheet lines, followed by an empty line:");

    let lines = line_reader();
    let checksum = lines.fold(0, |acc, line| {
        let nums = line.split("\t").filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>();

        // use get_line_checksum for first challenge
        // use get_line_division for second challenge
        acc + get_line_checksum(nums)
    });

    println!("Spreadsheet checksum is {}", checksum);
}
