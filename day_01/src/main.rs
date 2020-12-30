use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn main() {
    let filename = "src/input.txt";
    let constant = 2020;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data: Vec<u32> = reader.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect::<Result<_, _>>().unwrap();

    for (index, value) in data.iter().enumerate() {
        
        for (sub_index, second_value) in data.iter().enumerate() {
            if sub_index <= index {
                continue;
            }

            // println!("{}. {}", index, value);
            // println!("{}. {}", sub_index, second_value);
            // println!("----------------------------");

            if value + second_value == constant {
                println!("{} x {} = {}", value, second_value, value * second_value);
                std::process::exit(0);
            }

        }
    }

}