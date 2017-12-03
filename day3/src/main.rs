use std::io;

fn main() {
    println!("Please enter number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num = input.trim().parse::<i32>().expect("Failed to parse input");

    // use run_part_1 for first challenge
    // use run_part_2 for second challenge
    run_part_1(num);
}


#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

/* PART 1 */
fn get_ring_max(i: i32) -> i32 {
    let mul = i * 2 + 1;
    mul * mul
}

fn get_ring_number(num: i32) -> (i32, i32) {
    // first ring <= 1      1^2
    // second ring <= 9     3^2
    // third ring <= 25     5^2

    let mut i = 0;
    let mut last_ring_max = 0;
    loop {
        let ring_max = get_ring_max(i);
        if num <= ring_max {
            return (i, last_ring_max);
        }
        last_ring_max = ring_max;
        i += 1;
    }
}

fn get_number_pos(num: i32) -> Point {
    // this doesn't calculate properly for some reason
    if num == 1 {
        return Point { x: 0, y: 0 };
    }

    // instead of moving a cursor around in a spiral, we use some clever tricks here -
    // first, we calculate the current ring based on a pattern, then use that info to
    // figure out which side we're on, and our position
    let (ring, last_ring_max) = get_ring_number(num);

    let side_len = ring * 2;
    let rel_num = num - last_ring_max - 1;

    if rel_num < side_len {
        return Point { x: ring, y: rel_num + 1 - ring };
    }

    let rel_num = rel_num - side_len;
    if rel_num < side_len {
        return Point { x: ring - rel_num - 1, y: ring };
    }

    let rel_num = rel_num - side_len;
    if rel_num < side_len {
        return Point { x: -ring, y: ring - rel_num - 1 };
    }

    let rel_num = rel_num - side_len;
    Point { x: rel_num + 1 - ring, y: -ring }
}

fn get_number_dist(num: i32) -> i32 {
    let pos = get_number_pos(num);
    println!("Number position is {:?}", pos);

    pos.x.abs() + pos.y.abs()
}

fn run_part_1(num: i32) {
    println!("Distance is: {}", get_number_dist(num));
}

/* PART 2 */
const BOARD_WIDTH: usize = 25;
const BOARD_HEIGHT: usize = 25;

type Board = [[i32; BOARD_WIDTH]; BOARD_HEIGHT];

// so I learnt about lifetimes :D
fn get_cell_mut<'a>(board: &'a mut Board, p: (i32, i32)) -> &'a mut i32 {
    &mut board[(BOARD_HEIGHT as i32 / 2 + p.1) as usize][(BOARD_WIDTH as i32 / 2 + p.0) as usize]
}

fn get_cell<'a>(board: &'a Board, p: (i32, i32)) -> &'a i32 {
    &board[(BOARD_HEIGHT as i32 / 2 + p.1) as usize][(BOARD_WIDTH as i32 / 2 + p.0) as usize]
}

fn run_part_2(num: i32) {
    let mut board: Board = [[0; BOARD_HEIGHT]; BOARD_WIDTH];

    let mut current_p = (0, 0);
    let mut current_d = (1, 0);
    let mut current_ring = 0;

    // pre-seed board
    *get_cell_mut(&mut board, current_p) = 1;

    // move cursor around board and write to each position (nothing fancy here)
    loop {
        current_p.0 = current_p.0 + current_d.0;
        current_p.1 = current_p.1 + current_d.1;

        // change direction if needed
        if current_p.0 > current_ring {
            current_ring += 1;
            current_d = (0, 1);
        } else if current_p.1 + current_d.1 > current_ring {
            current_d = (-1, 0);
        } else if current_p.0 + current_d.0 < -current_ring {
            current_d = (0, -1);
        } else if current_p.1 + current_d.1 < -current_ring {
            current_d = (1, 0);
        }

        // get each adjacent (including diagonal) cell - ugly, I know
        let new_val = *get_cell(&board, (current_p.0 - 1, current_p.1))
                         + *get_cell(&board, (current_p.0 + 1, current_p.1))
                         + *get_cell(&board, (current_p.0, current_p.1 - 1))
                         + *get_cell(&board, (current_p.0, current_p.1 + 1))
                         + *get_cell(&board, (current_p.0 - 1, current_p.1 - 1))
                         + *get_cell(&board, (current_p.0 + 1, current_p.1 - 1))
                         + *get_cell(&board, (current_p.0 - 1, current_p.1 + 1))
                         + *get_cell(&board, (current_p.0 + 1, current_p.1 + 1));

        // update board and check for end condition
        *get_cell_mut(&mut board, current_p) = new_val;

        if new_val > num {
            println!("{} > {}, complete.", new_val, num);
            return;
        }
    }
}
