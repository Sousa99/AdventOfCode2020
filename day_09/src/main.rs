use std::fs::File;
use std::io::{BufRead, BufReader};

struct List {
    preamble : usize,
    numbers : Vec<u64>,
    invalid_found : Option<u64>,
}

impl List {
    pub fn new(preamble : usize) -> List {
        List {
            preamble : preamble,
            numbers : Vec::new(),
            invalid_found : None,
        }
    }

    fn get_invalid_number(&self) -> Option<u64> {
        return self.invalid_found.clone();
    }

    fn add(&mut self, value : u64) {
        let invalid_found_already : bool = match self.invalid_found {
            Some(_) => true,
            None => false,
        };

        if !invalid_found_already && self.numbers.len() >= self.preamble {
            if !self.check(value) { self.invalid_found = Some(value); }
        }

        self.numbers.push(value);
    }

    fn check(&self, value : u64) -> bool {
        let number_of_elements : usize = self.numbers.len();
        let first_to_consider : usize = number_of_elements - self.preamble;

        for index_i in first_to_consider..number_of_elements {
            for index_j in index_i..number_of_elements {
                if self.numbers[index_i] + self.numbers[index_j] == value {
                    return true;
                }
            }
        }

        return false;
    }

    fn find_indexes_that_sum(&self, value : u64) -> Result<(usize, usize), &'static str> {
        let number_of_elements : usize = self.numbers.len();

        for index_i in 0..number_of_elements {
            for index_j in (index_i + 1)..number_of_elements {

                let mut sum : u64 = 0;
                for index in index_i..(index_j + 1) {
                    sum = sum + self.numbers[index];
                }

                if sum == value { return Ok((index_i, index_j)); }
            }
        }

        return Err("No such number found!");
    }

    fn sum_max_min(&self, start : usize, end : usize) -> u64 {
        let mut min : u64 = self.numbers[start];
        let mut max : u64 = self.numbers[start];

        for index in start..(end + 1) {
            let value : u64 = self.numbers[index];
            if value < min { min = value; }
            if value > max { max = value; }
        }
        
        return min + max;
    }

    fn find_sum_between_sum_of_invalid(&self) -> Result<u64, &'static str> {
        let value : u64 = match self.invalid_found {
            Some(value) => value,
            None => return Err("There is no invalid number in list!"),
        };

        let (start, end) : (usize, usize) = match self.find_indexes_that_sum(value) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };

        return Ok(self.sum_max_min(start, end));
    }
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut list : List = List::new(25);

    for line in data.iter() {
        let value : u64 = match line.parse() {
            Ok(value) => value,
            Err(e) => panic!("{}", e),
        };

        list.add(value);
    }

    match list.get_invalid_number() {
        Some(value) => println!("The first value that could not be added: {}", value),
        None => println!("No invalid value found in list!"),
    }
    
    match list.find_sum_between_sum_of_invalid() {
        Ok(value) => println!("The weakness: {}", value),
        Err(e) => panic!(e),
    }
}
