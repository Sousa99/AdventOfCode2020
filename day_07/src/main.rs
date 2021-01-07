use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

// TODO:
// Reference to another bag in ContainedBag should be a pointer to Bag instead of its name
// Errors should not be a simple &str

const BAG_NAME : &str = "shiny gold";

struct Bag {
    bag_name : String,
    sub_bags : Vec<ContainedBag>,
    visited : bool
}

impl Bag {
    pub fn new(bag_name : String) -> Bag {
        Bag {
            bag_name : bag_name,
            sub_bags : Vec::new(),
            visited : false,
        }
    }

    fn get_bag_name(&self) -> String {
        return self.bag_name.clone();
    }

    fn get_sub_bags(&self) -> Vec<ContainedBag> {
        return (&self).sub_bags.clone();
    }

    fn add_contain_bag(&mut self, quantity : u32, sub_bag_name : String) {
        let new_sub_bag = ContainedBag::new(quantity, sub_bag_name);
        self.sub_bags.push(new_sub_bag);
    }

    fn bag_visited(&self) -> bool {
        return self.visited;
    }

    fn set_visited(&mut self, value : bool) {
        self.visited = value;
    }

    fn sub_bags_contain(&self, sub_bag_name : String) -> bool {
        return self.sub_bags.iter()
            .any(|sub_bag| sub_bag.get_sub_bag_name() == sub_bag_name);
    }
}

#[derive(Clone, Debug)]
struct ContainedBag {
    quantity : u32,
    bag_ref : String
}

impl ContainedBag {
    pub fn new(quantity : u32, bag_ref : String) -> ContainedBag {
        ContainedBag {
            quantity : quantity,
            bag_ref : bag_ref
        }
    }

    fn get_sub_bag_quantity(&self) -> u32 {
        return self.quantity;
    }

    fn get_sub_bag_name(&self) -> String {
        return self.bag_ref.clone();
    }
}

struct Rules {
    bags : Vec<Bag>
}

impl Rules {
    pub fn new() -> Rules {
        Rules {
            bags : Vec::new()
        }
    }

    fn add_bag(&mut self, bag_name : String) {
        if !self.bag_exists(bag_name.clone()) {
            let new_bag = Bag::new(bag_name);
            self.bags.push(new_bag);
        }
    }

    fn bag_exists(&self, bag_name : String) -> bool {
        let exists : bool = self.bags.iter()
            .any(|bag| bag.get_bag_name() == bag_name);

        return exists;
    }

    fn add_contain_to_bag(&mut self, bag_name : String, quantity : u32, sub_bag_name : String) -> Result<&'static str, &'static str> {
        for bag in self.bags.iter_mut() {
            if bag.get_bag_name() == bag_name {
                bag.add_contain_bag(quantity, sub_bag_name);
                return Ok("Contained added sucessfully");
            }
        }

        return Err("No bag with such name found!");
    }

    fn number_bags_can_contain(&mut self, bag_name : String) -> u32 {
        let mut number_bags : u32 = 0;
        let mut bags_to_explore : Vec<String> = Vec::new();

        for bag in self.bags.iter_mut() {
            if bag.bag_visited() { continue; }
            if bag.sub_bags_contain(bag_name.clone()) {
                bag.set_visited(true);
                number_bags = number_bags + 1;
                bags_to_explore.push(bag.get_bag_name());
            }
        }

        for bag_name in bags_to_explore {
            number_bags = number_bags + self.number_bags_can_contain(bag_name);
        }

        return number_bags;
    }

    fn number_bags_inside(&mut self, bag_name : String) -> Result<u32, &'static str> {
        let mut number_bags : u32 = 0;
        let mut contains : Option<Vec<ContainedBag>> = None;

        for bag in self.bags.iter_mut() {
            if bag.get_bag_name() == bag_name {
                contains = Some(bag.get_sub_bags());
                break;
            }
        }

        let contains = match contains {
            Some(value) => value,
            None => {
                println!("Bag that originated error: {}", bag_name);
                return Err("Didn't find rule for a bag");
            }
        };

        for contain in contains.iter() {
            let sub_bag_name : String = contain.get_sub_bag_name();
            let number_bags_inside_sub : u32 = match self.number_bags_inside(sub_bag_name) {
                Ok(number) => number,
                Err(e) => return Err(e),
            };

            number_bags = number_bags + contain.get_sub_bag_quantity() * (1 + number_bags_inside_sub);
        }

        return Ok(number_bags);
    }

    fn clean_bags_visited(&mut self) {
        for bag in self.bags.iter_mut() {
            bag.set_visited(false);
        }
    }
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let regex_doesnt_contain = Regex::new(r"^(.+) bags contain no other bags.$").unwrap();
    let regex_contain_sep = Regex::new(r"^(.+) bags contain( (?:\d+) (?:.+) (?:bag|bags)(?:,|\.))+$").unwrap(); 
    let regex_contain_each = Regex::new(r"^(\d+) (.+) (?:bag|bags)+$").unwrap();

    let mut rules : Rules = Rules::new();

    for line in data.iter() {
        if regex_contain_sep.is_match(line) {
            let cap : regex::Captures = regex_contain_sep.captures(line).unwrap();

            let bag_name : String = (&cap[1]).to_string();
            let contains : String = (&cap[2]).trim().replace(".", "").to_string();
            
            rules.add_bag(bag_name.clone());

            let contains_split = contains.split(", ");
            for contain in contains_split {
                let sub_cap : regex::Captures = regex_contain_each.captures(contain).unwrap();

                let quantity : u32 = (&sub_cap[1]).to_string().parse().unwrap();
                let sub_bag_name : String = (&sub_cap[2]).to_string();

                match rules.add_contain_to_bag(bag_name.clone(), quantity, sub_bag_name) {
                    Ok(_) => (),
                    Err(e) => println!("{}", e),
                };
            }

        } else if regex_doesnt_contain.is_match(line) {
            let cap : regex::Captures = regex_doesnt_contain.captures(line).unwrap();

            let bag_name : String = (&cap[1]).to_string();
            rules.add_bag(bag_name);

        } else {
            println!("Line wasn't parsed: {}", line);
        }
    }

    let number_bags_can_contain : u32 = rules.number_bags_can_contain(BAG_NAME.to_string());
    println!("Number of bags that contain '{}': {}", BAG_NAME.to_string(), number_bags_can_contain);
    rules.clean_bags_visited();
    
    let number_bags_inside : u32 = match rules.number_bags_inside(BAG_NAME.to_string()) {
        Ok(value) => value,
        Err(e) => panic!(e)
    };

    println!("Number of bags inside '{}': {}", BAG_NAME.to_string(), number_bags_inside);
}