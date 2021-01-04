use std::fs::File;
use std::io::{BufRead, BufReader};

struct Seat {
    row : u32,
    collumn : u32,
    id : i32,
}

impl Seat {
    fn compute_id(&mut self) {
        self.id = formula_id(self.row, self.collumn) as i32;
    }
}

fn build_seat(row : u32, collumn : u32) -> Seat {
    let mut new_seat = Seat {
        row : row,
        collumn : collumn,
        id : -1,
    };

    new_seat.compute_id();
    return new_seat;
}

const MAX_ROW : u32= 127;
const MAX_COLLUMN : u32 = 7;
const DIVISION_INDEX : usize = 7;

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut seen_seats : Vec<Seat> = Vec::new();
    for line in data.iter() {
        let (row_code, collumn_code) = line.split_at(DIVISION_INDEX);

        let row = compute_row(row_code.to_string());
        let collumn = compute_collumn(collumn_code.to_string());

        let new_seat = build_seat(row, collumn);
        seen_seats.push(new_seat);
    }

    let max_id : i32 = find_biggest_id(&seen_seats);
    println!("Seat with biggest id: {}", max_id);
    let missing_id : i32 = find_free_space(&seen_seats);
    println!("Free space on the plane: {}", missing_id);
}

fn compute_row(value : String) -> u32 {
    const LOWER_HALF : char = 'F';
    const UPPER_HALF : char = 'B';

    let mut min : u32 = 0;
    let mut max : u32 = MAX_ROW;

    for character in value.chars() {
        let mid_point : f32 = min as f32 + (max as f32 - min as f32) / 2.0;
        if character == LOWER_HALF { max = mid_point.floor() as u32; }
        else if character == UPPER_HALF { min = mid_point.ceil() as u32; }
    }

    assert_eq!(min, max);
    return min;
}

fn compute_collumn(value : String) -> u32 {
    const LOWER_HALF : char = 'L';
    const UPPER_HALF : char = 'R';

    let mut min : u32 = 0;
    let mut max : u32 = MAX_COLLUMN;

    for character in value.chars() {
        let mid_point : f32 = min as f32 + (max as f32 - min as f32) / 2.0;
        if character == LOWER_HALF { max = mid_point.floor() as u32; }
        else if character == UPPER_HALF { min = mid_point.ceil() as u32; }
    }

    assert_eq!(min, max);
    return min;
}

fn find_biggest_id(seats : &Vec<Seat>) -> i32 {
    let mut max_id : i32 = -1;

    for seat in seats.iter() {
        if seat.id > max_id { max_id = seat.id; }
    }

    return max_id;
}

fn formula_id(row : u32, collumn : u32) -> u32 {
    return row * 8 + collumn;
}

fn find_free_space(seats : &Vec<Seat>) -> i32 {
    let mut min : i32 = -1;
    let mut max : i32 = -1;
    let mut sum : i32 = 0;

    for seat in seats.iter() {
        sum = sum + seat.id;
        if seat.id == 577 { println!("Found It"); }

        if min == -1 || seat.id < min { min = seat.id; }
        if max == -1 || seat.id > max { max = seat.id; }
    }

    let min_sum_total = compute_sum_till_n(min - 1);
    let max_sum_total = compute_sum_till_n(max);
    println!("Min Total = {}", min_sum_total);
    println!("Sum = {}", sum);
    println!("Sum Total = {}", max_sum_total);

    let missing_value : i32 = max_sum_total - min_sum_total - sum;
    return missing_value;
}

fn compute_sum_till_n(n : i32) -> i32 {
    return n * (n + 1) / 2;
}