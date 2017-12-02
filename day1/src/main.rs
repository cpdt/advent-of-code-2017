use std::io;

fn get_sum(nums: Vec<u32>, skip: usize) -> u32 {
    nums.iter().enumerate().fold(0, |acc, (i, num)| {
        let next_index = (i + skip) % nums.len();
        let next_num = nums[next_index];

        if *num == next_num {
            acc + num
        } else {
            acc
        }
    })
}

fn main() {
    println!("Please enter the number:");

    let mut val = String::new();
    io::stdin().read_line(&mut val).expect("Failed to read line");

    let nums = val.chars().filter_map(|x| x.to_digit(10)).collect::<Vec<u32>>();

    // skip = 1 for first challenge
    // skip = nums.len() / 2 for second challenge
    let sum = get_sum(nums, 1);

    println!("Sum is {}", sum);
}
