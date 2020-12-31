use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

/* TODO: Implementation could definitelly be more efficient:
- Verifying only elements after the current one on sub loops
- Checking at the same time for 2 and 3 elements
- (Maybe) build an implementation that runs x loops based on a variable instead of hard coded the number of elements
*/

fn main() {
    let filename = "src/input.txt";
    let constant = 2020;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data: Vec<u32> = reader.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect::<Result<_, _>>().unwrap();

    // FIND TWO NUMBERS - PART 1
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
            }
        }
    }

    // FIND THREE NUMBERS - PART 1
    for (index, value) in data.iter().enumerate() {     
        for (sub_index, second_value) in data.iter().enumerate() {
            if sub_index <= index {
                continue;
            }
            for (sub_sub_index, third_value) in data.iter().enumerate() {
                if sub_sub_index <= sub_index {
                    continue;
                }

                if value + second_value + third_value == constant {
                    println!("{} x {} x {} = {}", value, second_value, third_value, value * second_value * third_value);
                    std::process::exit(0);
                }
            }
        }
    }

}