use std::io;

fn get_line_checksum(nums: &Vec<i32>) -> i32 {
    let max_num = nums.iter().fold(0, |acc, &x| std::cmp::max(acc, x));
    let min_num = nums.iter().fold(max_num, |acc, &x| std::cmp::min(acc, x));

    max_num - min_num
}

fn get_line_division(nums: &Vec<i32>) -> i32 {
    for (i, a) in nums.iter().enumerate() {
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

fn main() {
    println!("Please enter the spreadsheet lines, followed by an empty line:");

    let mut checksum = 0;
    let mut val = String::new();
    while {
        io::stdin().read_line(&mut val).expect("Failed to read line");
        val.len() > 1
    } {
        let nums = val.trim().split("\t").filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>();

        // use get_line_checksum for first challenge
        // use get_line_division for second challenge
        checksum += get_line_checksum(&nums);
        val.clear();
    }

    println!("Spreadsheet checksum is {}", checksum);
}
