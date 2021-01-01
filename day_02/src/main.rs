use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data: Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut valid_passwords_1 : Vec<String> =  Vec::new();
    let mut valid_passwords_2 : Vec<String> =  Vec::new();

    for line in data.iter() {
        let split = line.split(" ").collect::<Vec<&str>>();
        //println!("{} : {} : {}", split[0], split[1], split[2]);

        let split_limit = split[0].split("-").collect::<Vec<&str>>();
        let lower_limit: u32 = split_limit[0].parse().unwrap();
        let upper_limit: u32 = split_limit[1].parse().unwrap();

        let mut pattern = split[1].to_string();
        pattern.pop();

        let password = split[2].to_string();
        
        let mut count : u32 = 0;
        let mut count_specific : u32 = 0;
        for (index, char) in password.chars().enumerate() {
            if char.to_string() == pattern { count = count + 1; }

            if char.to_string() == pattern && 
                (index as u32 == lower_limit - 1 || index as u32 == upper_limit - 1) {
                    count_specific = count_specific + 1; 
                }
        }

        let valid = valid_sub_pattern(lower_limit, upper_limit, count);
        if valid { valid_passwords_1.push(password.clone()); }

        let valid = count_specific == 1;
        if valid { valid_passwords_2.push(password.clone()); }
    }

    println!("Number of correct passwords (part 1): {}", valid_passwords_1.len());
    println!("Number of correct passwords (part 2): {}", valid_passwords_2.len());
}

fn valid_sub_pattern(lower_limit : u32, upper_limit : u32, count : u32) -> bool {
    return lower_limit <= count && count <= upper_limit;
}
