use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

// ------------------ Phase ------------------
enum Phase {
    FieldsInput,
    MyTicket,
    NearbyTickets
}

// ------------------ Rule ------------------
struct Rule {
    lower_limit : u32,
    upper_limit : u32,
}

impl Rule {
    pub fn new(lower_limit : u32, upper_limit : u32) -> Rule {
        Rule {
            lower_limit : lower_limit,
            upper_limit : upper_limit,
        }
    }

    fn valid_value(&self, value : u32) -> bool { value >= self.lower_limit && value <= self.upper_limit }
}

// ------------------ Field ------------------
struct Field {
    name : String,
    rules : Vec<Rule>
}

impl Field {
    pub fn new(name : String) -> Field {
        Field {
            name : name,
            rules : Vec::new(),
        }
    }

    fn get_name(&self) -> String { self.name.clone() }
    fn add_rule(&mut self, lower_limit : u32, upper_limit : u32) { self.rules.push(Rule::new(lower_limit, upper_limit)) }
    fn valid_value(&self, value : u32) -> bool {
        for rule in self.rules.iter() {
            if rule.valid_value(value) { return true; }
        }

        return false;
    }
}

// ------------------ Ticket ------------------
struct Ticket {
    values : Vec<u32>
}

impl Ticket {
    pub fn new() -> Ticket {
        Ticket {
            values : Vec::new(),
        }
    }

    fn add_value(&mut self, value : u32) { self.values.push(value) }
    fn get_value(&self, at : usize) -> u32 { self.values[at] }
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut current_phase : Phase = Phase::FieldsInput;
    let mut fields : Vec<Field> = Vec::new();
    let mut invalid_values : Vec<u32> = Vec::new();
    let mut your_ticket : Ticket = Ticket::new();
    let mut nearby_tickets : Vec<Ticket> = Vec::new();
    let mut possible_field_correspondence : HashMap<usize, Vec<String>> = HashMap::new(); 

    for line in data.iter() {

        // When blank like is encountered update current phase
        if line == "" {
            current_phase = match current_phase {
                Phase::FieldsInput => Phase::MyTicket,
                Phase::MyTicket => Phase::NearbyTickets,
                Phase::NearbyTickets => Phase::NearbyTickets,
            };

            continue;
        }

        // Check if line must be ignored
        match current_phase {
            Phase::FieldsInput => (),
            Phase::MyTicket if line == "your ticket:" => continue,
            Phase::NearbyTickets if line == "nearby tickets:" => continue,
            _ => (),
        }

        // Actually the code
        match current_phase {
            Phase::FieldsInput => {
                let split : Vec<&str> = line.split(": ").collect::<Vec<&str>>();

                let field_name : String = split[0].to_string();
                let rules_string : String = split[1].replace(" or ", " ");

                let mut new_field : Field = Field::new(field_name);
                
                for rule_string in rules_string.split(" ") {
                    let rule_split : Vec<&str> = rule_string.split("-").collect::<Vec<&str>>();
                    let lower_limit : u32 = rule_split[0].parse().unwrap();
                    let upper_limit : u32 = rule_split[1].parse().unwrap();

                    new_field.add_rule(lower_limit, upper_limit);
                }

                fields.push(new_field);
            },
            Phase::MyTicket => {
                for value in line.split(",") {
                    let value_parsed : u32 = value.parse().unwrap();
                    your_ticket.add_value(value_parsed);
                }
            },
            Phase::NearbyTickets => {
                let mut new_ticket : Ticket = Ticket::new();
                let mut valid_ticket : bool = true;

                for value in line.split(",") {
                    let value_parsed : u32 = value.parse().unwrap();
                    let mut valid_value : bool = false;
                    new_ticket.add_value(value_parsed);
                    
                    for field in fields.iter() {
                        if field.valid_value(value_parsed) {
                            valid_value = true;
                            break;
                        }
                    }

                    if !valid_value {
                        invalid_values.push(value_parsed);
                        valid_ticket = false;
                    }
                }

                if valid_ticket { nearby_tickets.push(new_ticket) }
            },
        }
    }

    // Part 1
    let mut sum : u32 = 0;
    for value in invalid_values.into_iter() { sum = sum + value }
    println!("Error rate is '{}' (Part 1)", sum);

    // Part 2
    let number_of_fields : usize = fields.len();
    for index_field in 0..number_of_fields {
        let mut valid_fields : Vec<String> = Vec::new();

        for field in fields.iter() {
            let mut valid_field : bool = true;
            for ticket in nearby_tickets.iter() {
                if !field.valid_value(ticket.get_value(index_field)) {
                    valid_field = false;
                    break;
                }
            }

            if valid_field { valid_fields.push(field.get_name())}
        }

        possible_field_correspondence.insert(index_field, valid_fields);
    }

    let mut fields_set : HashMap<usize, String> = HashMap::new();
    loop {
        // Figure out which field only have one available correspondence
        let mut to_remove : Vec<usize> = Vec::new();
        for (&index, fields) in &possible_field_correspondence {
            if fields.len() == 1 {
                let field_index : usize = index;
                let field_name : String = fields.first().unwrap().to_string();
                fields_set.insert(field_index, field_name);

                to_remove.push(field_index);
            }
        }

        // If no field is found to remove
        if to_remove.len() == 0 { break }

        // Remove from correspondence fields already set
        for remove_index in to_remove {
            possible_field_correspondence.remove(&remove_index);
        }

        // Remove from correspondence fields already set
        for (_, fields) in &mut possible_field_correspondence {
            let mut indexes_to_remove : Vec<usize> = Vec::new();
            for (index_of_field, field) in fields.iter().enumerate() {
                if fields_set.values().any(|set_field| set_field == field) {
                    indexes_to_remove.push(index_of_field);
                }
            }

            for index_to_remove in indexes_to_remove.into_iter() {
                fields.remove(index_to_remove);
            }
        }
    }

    // Print out correspondence between attributes and find out attributtes that match challenge
    println!("---------------------");
    let mut indexes_to_multiply : Vec<usize> = Vec::new();
    for index in 0..fields_set.len() {
        let field_name : String = fields_set.get(&index).unwrap().to_string();
        println!("Field {}: '{}'", index, field_name);
        if field_name.contains("departure") { indexes_to_multiply.push(index) }
    }
    println!("---------------------");
    
    // Find result
    let mut multiplication : u64 = 1;
    for index in indexes_to_multiply.into_iter() { multiplication = multiplication * your_ticket.get_value(index) as u64 }
    println!("The result is '{}' (Part 2)", multiplication);
}
