use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

struct Group {
    at_least_one_yes : HashSet<char>,
    all_yes : HashSet<char>
}

impl Group {
    pub fn new() -> Group {
        Group {
            at_least_one_yes : HashSet::new(),
            all_yes : HashSet::new(),
        }
    }
    
    fn get_all(&self) -> HashSet<char> {
        return self.all_yes.clone();
    }

    fn insert_at_least_one(&mut self, elem : char) {
        self.at_least_one_yes.insert(elem);
    }
    fn insert_all(&mut self, elem : char) {
        self.all_yes.insert(elem);
    }
    fn remove_all(&mut self, elem : char) {
        self.all_yes.remove(&elem);
    }

    fn number_questions_at_least_one_yes(&self) -> usize {
        return self.at_least_one_yes.len();
    }
    fn number_questions_all_yes(&self) -> usize {
        return self.all_yes.len();
    }
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut groups : Vec<Group> = Vec::new();
    let mut new_group : Group = Group::new();
    let mut first_person_in_group : bool = true;
    for line in data.iter() {
        let char_count = line.chars().count();
        if char_count == 0 {
            groups.push(new_group);
            new_group = Group::new();
            first_person_in_group = true;
            continue;
        }

        let mut to_keep : HashSet<char> = HashSet::new();
        for characther in line.chars() {
            to_keep.insert(characther);
            new_group.insert_at_least_one(characther);
        }

        // If first person in group add all
        if first_person_in_group {
            for characther in to_keep {
                new_group.insert_all(characther);
            }
        // Else remove elements present not in new persons list
        } else {
            for characther in new_group.get_all() {
                if ! to_keep.contains(&characther) { new_group.remove_all(characther); }
            }
        }
        
        // Update not first person in group anymore
        first_person_in_group = false;
    }
    groups.push(new_group);

    let mut sum_at_least_one : u32 = 0;
    let mut sum_all : u32 = 0;
    for group in groups.iter() {
        sum_at_least_one = sum_at_least_one + group.number_questions_at_least_one_yes() as u32;
        sum_all = sum_all + group.number_questions_all_yes() as u32;
    }

    println!("At least one yes: {}", sum_at_least_one);
    println!("All yes: {}", sum_all);

}
