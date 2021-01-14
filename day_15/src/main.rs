use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

type Item = u32;
type Age = usize;

// ---------------- Item ----------------
#[derive(Clone)]
struct Record {
    item : Item,
    ages : (Option<Age>, Option<Age>)
}

impl Record {
    pub fn new(item : Item) -> Record {
        Record {
            item : item,
            ages : (None, None),
        }
    }

    fn get_item(&self) -> Item { self.item }
    fn add_age(&mut self, age : Age) {
        // Update Values
        if self.ages.0.is_none() { self.ages.0 = Some(age); }
        else if self.ages.1.is_none() { self.ages.1 = Some(age); }
        else { self.ages.0 = self.ages.1; self.ages.1 = Some(age); }
    }

    fn spoken_before(&self) -> bool { self.ages.0.is_some() && self.ages.1.is_some() }
    fn get_age_between(&self) -> Age { self.ages.1.unwrap() - self.ages.0.unwrap() }
}

// ---------------- Game ----------------
struct Game {
    current_age : Age,
    last_item : Option<Record>,
    records : HashMap<Item, Record>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            current_age : 1,
            last_item : None,
            records : HashMap::with_capacity(5000000),
        }
    }

    fn get_last_item(&self) -> Option<Record> { self.last_item.clone() }
    fn get_current_age(&self) -> Age { self.current_age }
    fn increment_age(&mut self) { self.current_age = self.current_age + 1 }

    fn add_item(&mut self, item : Item) {
        let age_before_inc : Age = self.current_age;
        self.increment_age();

        match self.records.get_mut(&item) {
            Some(record) => {
                record.add_age(age_before_inc);
                self.last_item = Some(record.clone())
            },
            None => {
                let mut new_record : Record = Record::new(item);
                new_record.add_age(age_before_inc);

                self.last_item = Some(new_record.clone());
                self.records.insert(item, new_record);
            }
        }
    }

    fn run_iteration(&mut self) {
        match &self.last_item {
            None => panic!("No last item set! (should not happen)"),
            Some(record) => {
                let mut next_item : Option<Item> = None;

                // Discover if number was spoken before
                if record.spoken_before() { next_item = Some(record.get_age_between() as Item); }

                match next_item {
                    None => self.add_item(0),
                    Some(item) => self.add_item(item),
                }


            }
        }
    }
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();
    
    let line : String = data[0].clone();
    let split : std::str::Split<&str> = line.split(",");
    let mut game : Game = Game::new();

    for item in split {
        let item_value : Item = match item.parse() {
            Ok(value) => value,
            Err(_) => panic!("Could not convert the value."),
        };

        game.add_item(item_value);
    }

    // Part 1
    let go_until : Age = 2020;
    while game.get_current_age() - 1 != go_until { game.run_iteration(); }
    match game.get_last_item() {
        Some(item) => println!("The last item was '{}' (part 1)", item.get_item()),
        None => println!("The game didn't even start!"),
    }

    // Part 2
    let go_until : Age = 30000000;
    while game.get_current_age() - 1 != go_until { game.run_iteration(); }
    match game.get_last_item() {
        Some(item) => println!("The last item was '{}' (part 2)", item.get_item()),
        None => println!("The game didn't even start!"),
    }

}
