use std::io;
use std::collections::HashSet;

fn do_redistribute(blocks: &mut Vec<i32>) {
    let iter_max = blocks.iter().enumerate().fold((0, 0), |acc, (i, &x)| {
        if x > acc.0 { (x, i) } else { acc }
    });

    blocks[iter_max.1] = 0;
    let mut remaining = iter_max.0;
    let mut i = iter_max.1;

    while remaining > 0 {
        i = (i + 1) % blocks.len();

        blocks[i] += 1;
        remaining -= 1;
    }

    let joined = blocks.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    println!("{}", joined.join("\t"));
}

fn find_distribute_repeats(blocks: &mut Vec<i32>) -> i32 {
    let mut found_dist = HashSet::new();

    let mut i = 0;
    while !found_dist.contains(blocks) {
        found_dist.insert(blocks.to_vec());
        do_redistribute(blocks);
        i += 1;
    }

    i
}

fn main() {
    println!("Please enter the blocks separated by a tab:");

    let mut val = String::new();
    io::stdin().read_line(&mut val).expect("Failed to read line");

    let mut nums = val.split("\t").filter_map(|x| x.trim().parse::<i32>().ok()).collect::<Vec<i32>>();

    let repeats_1 = find_distribute_repeats(&mut nums);
    let repeats_2 = find_distribute_repeats(&mut nums);

    // repeats_1 is answer to part 1, repeats_2 to part 2
    println!("Repeat takes {} iterations, and occurs again after {} iterations", repeats_1, repeats_2);
}
