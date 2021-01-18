use std::fs::File;
use std::io::{BufRead, BufReader};

// TODO:
// Refactor code to have less copies of itself and improve readibility
// (This part was done in a hurry)

// ------------------- Rule -------------------
#[derive(Copy, Clone)]
enum Rule {
    Part1,
    Part2,
}

fn solve_parenthesis(mut slice : Vec<String>, rule : Rule) -> Vec<String> {
    let mut open_parenthesis : usize = 0;
    let mut close_parenthesis : usize = 0;

    for (index, element) in slice.iter().enumerate() {
        if element == "(" { open_parenthesis = index }
        else if element == ")" {
            close_parenthesis = index;
            break;
        }
    }

    let solve_slice : Vec<String> = (&slice[(open_parenthesis + 1)..close_parenthesis]).to_vec();
    let result : i64 = compute_result(solve_slice, rule);

    // Remove dealt with values
    for _ in 0..(close_parenthesis - open_parenthesis + 1) { slice.remove(open_parenthesis); }
    // Adding result in correct place
    slice.insert(open_parenthesis, result.to_string());

    return slice;
}

fn solve_addition(mut slice : Vec<String>) -> Vec<String> {
    let mut addition_index : usize = 0;

    for (index, element) in slice.iter().enumerate() {
        if element == "+" {
            addition_index = index;
            break;
        }
    }

    let value_before : i64 = slice[addition_index - 1].parse().unwrap();
    let value_after : i64 = slice[addition_index + 1].parse().unwrap();
    let result : i64 = value_before + value_after;

    // Remove dealt with values
    for _ in 0..3 { slice.remove(addition_index - 1); }
    // Adding result in correct place
    slice.insert(addition_index - 1, result.to_string());

    return slice;
}

fn solve_multiplication(mut slice : Vec<String>) -> Vec<String> {
    let mut multiplication_index : usize = 0;

    for (index, element) in slice.iter().enumerate() {
        if element == "*" {
            multiplication_index = index;
            break;
        }
    }

    let value_before : i64 = slice[multiplication_index - 1].parse().unwrap();
    let value_after : i64 = slice[multiplication_index + 1].parse().unwrap();
    let result : i64 = value_before * value_after;

    // Remove dealt with values
    for _ in 0..3 { slice.remove(multiplication_index - 1); }
    // Adding result in correct place
    slice.insert(multiplication_index - 1, result.to_string());

    return slice;
}

fn solve_addition_and_multiplication(mut slice : Vec<String>) -> Vec<String> {
    let mut operation_index : usize = 0;
    let mut symbol : &str = "";

    for (index, element) in slice.iter().enumerate() {
        if element == "*" || element == "+" {
            operation_index = index;
            symbol = element;
            break;
        }
    }

    let value_before : i64 = slice[operation_index - 1].parse().unwrap();
    let value_after : i64 = slice[operation_index + 1].parse().unwrap();
    let result : i64 = match symbol {
        "+" => value_before + value_after,
        "*" => value_before * value_after,
        _ => panic!("Symbol not recognized"),
    };

    // Remove dealt with values
    for _ in 0..3 { slice.remove(operation_index - 1); }
    // Adding result in correct place
    slice.insert(operation_index - 1, result.to_string());

    return slice;
}

fn compute_result(mut slice : Vec<String>, rule : Rule) -> i64 {
    while slice.contains(&String::from("(")) { slice = solve_parenthesis(slice, rule) }

    match rule {
        Rule::Part1 => {
            while slice.contains(&String::from("+")) || slice.contains(&String::from("*")) {
                slice = solve_addition_and_multiplication(slice);
            }
        },
        Rule::Part2 => {
            while slice.contains(&String::from("+")) { slice = solve_addition(slice) }
            while slice.contains(&String::from("*")) { slice = solve_multiplication(slice) }
        }
    }

    if slice.len() == 1 {
        return slice[0].parse().unwrap();
    }

    println!("{:?}", slice);

    panic!("It should never have gotten here!");
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut sum_part1 : i64 = 0;
    let mut sum_part2 : i64 = 0;
    for line in data.iter() {
        let corrected_line : String = line.replace("(", "( ").replace(")", " )");
        let characthers : Vec<String> = corrected_line.split_whitespace()
            .map(|s| s.to_string()).collect();
        
        let result : i64 = compute_result(characthers.clone(), Rule::Part1);
        sum_part1 = sum_part1 + result;
        let result : i64 = compute_result(characthers, Rule::Part2);
        sum_part2 = sum_part2 + result;
    }

    println!("Result = {} (part 1)", sum_part1);
    println!("Result = {} (part 2)", sum_part2);
}
