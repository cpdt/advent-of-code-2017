use std::io;

fn find_exit_steps(lines: &mut Vec<i32>, do_decrement: bool) -> i32 {
    let mut pos: i32 = 0;
    let mut i: i32 = 0;

    while pos >= 0 && pos < lines.len() as i32 {
        let p = &mut lines[pos as usize];
        pos += *p;

        if do_decrement && *p >= 3 {
            *p -= 1;
        } else {
            *p += 1;
        }

        i += 1;
    };

    i
}

struct LineNumberReader;

impl Iterator for LineNumberReader {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let mut val = String::new();
        match io::stdin().read_line(&mut val) {
            Err(_) => None,
            Ok(_) => {
                let trimmed_val = val.trim();

                if trimmed_val.len() > 0 {
                    trimmed_val.parse::<i32>().ok()
                } else {
                    None
                }
            }
        }
    }
}

fn line_num_reader() -> LineNumberReader {
    LineNumberReader
}

fn main() {
    println!("Enter maze lines, followed by an empty line:");

    // set to false for challenge 1, true for challenge 2
    const DO_DECREMENT: bool = false;

    let mut lines = line_num_reader().collect::<Vec<i32>>();
    let steps = find_exit_steps(&mut lines, DO_DECREMENT);

    println!("It took {} steps to exit the maze.", steps);
}
