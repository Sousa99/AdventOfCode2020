use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

const DEBUG : bool = false;

// For simplification instead of creating non-needed struct, set types
type Cup = u64;

// ---------------- Game ----------------
struct Game {
    // Some game consts
    pickup_next : u32,
    // State Game
    current_move : u32,
    cups : HashMap<Cup, Cup>,
    current_cup : Cup,
    // Auxiliary Variables
    last_added : Cup,
    lowest_cup : Cup,
    highest_cup : Cup,
}

impl Game {
    pub fn new(pickup_next : u32) -> Game {
        Game {
            pickup_next : pickup_next,

            current_move : 1,
            cups : HashMap::new(),
            current_cup : 0,

            last_added : 0,
            lowest_cup : 0,
            highest_cup : 0,
        }
    }

    fn add_cup(&mut self, cup : Cup) {
        if self.current_cup == 0 {
            self.current_cup = cup;
            self.last_added = cup;
            self.lowest_cup = cup;
            self.highest_cup = cup;
            return;
        }
        
        if cup > self.highest_cup { self.highest_cup = cup }
        else if cup < self.lowest_cup { self.lowest_cup = cup }

        self.cups.insert(self.last_added, cup);
        self.last_added = cup;
    }

    fn finish_cycle(&mut self) { self.cups.insert(self.last_added, self.current_cup); }

    fn create_missing_cups_until(&mut self, value : Cup) {
        for new_value in (self.highest_cup + 1)..(value + 1) {
            self.add_cup(new_value);
        }

        self.finish_cycle();
    }

    fn make_move(&mut self) {
        if DEBUG { println!("-- move {} --", self.current_move) };
        self.current_move = self.current_move + 1;

        // Print current state
        if DEBUG {
            let mut all_cups : Vec<Cup> = Vec::new();
            let mut tmp_cup : Cup = self.cups.get(&self.current_cup).unwrap().clone();
            all_cups.push(self.current_cup);
            while tmp_cup != self.current_cup {
                all_cups.push(tmp_cup);
                tmp_cup = self.cups.get(&tmp_cup).unwrap().clone();
            }

            let all_cups : String = all_cups.iter()
                .map(|cup| cup.to_string()).collect::<Vec<String>>().join(" ");
            println!("cups: {}", all_cups);
        }

        // Pickup cups
        let mut pickedup_cups : Vec<Cup> = Vec::new();
        let mut tmp_cup : Cup = self.cups.get(&self.current_cup).unwrap().clone();
        for _ in 0..self.pickup_next {
            pickedup_cups.push(tmp_cup);
            tmp_cup = self.cups.remove(&tmp_cup).unwrap().clone();
        }
        self.cups.insert(self.current_cup, tmp_cup);

        // Print picked up cups
        if DEBUG {
            let pickedup_cups_print : String = pickedup_cups.iter()
                .map(|cup| cup.to_string()).collect::<Vec<String>>().join(", ");
            println!("pick up: {}", pickedup_cups_print);
        }

        // Place picked cups
        let mut cups_placed : bool = false;
        let mut cup_to_find : Cup = self.current_cup - 1;
        while !cups_placed {
            // If value is lower than lowest cup, jumpt to highest value
            if cup_to_find < self.lowest_cup {
                cup_to_find = self.highest_cup;
            }

            match self.cups.get(&cup_to_find) {
                Some(&next_cup) => {
                    if DEBUG { println!("destination: {}", cup_to_find) }

                    let first_picked_up : Cup = pickedup_cups.remove(0);
                    self.cups.insert(cup_to_find, first_picked_up);

                    let mut tmp_cup : Cup = first_picked_up;
                    for _ in 0..pickedup_cups.len() {
                        let cup : Cup = pickedup_cups.remove(0);
                        self.cups.insert(tmp_cup, cup);
                        tmp_cup = cup;
                    }

                    self.cups.insert(tmp_cup, next_cup);
                    cups_placed = true;
                },

                None => cup_to_find = cup_to_find - 1,
            }
        }

        // Change current cup
        self.current_cup = self.cups.get(&self.current_cup).unwrap().clone();

    }

    fn get_result(&self, cup_from : Cup) -> Vec<Cup> {
        let mut result : Vec<Cup> = Vec::new();
        let mut next_cup : Cup = self.cups.get(&cup_from).unwrap().clone();

        while next_cup != cup_from {
            result.push(next_cup);
            next_cup = self.cups.get(&next_cup).unwrap().clone();
        }

        return result;
    }
}

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut game_1 : Game = Game::new(3);
    let mut game_2 : Game = Game::new(3);
    for line in data.iter() {
        for characther in line.chars() {
            // Parse Cup
            let cup : Result<Cup, _> = characther.to_string().parse();
            assert!(cup.is_ok());
            let cup : Cup = cup.unwrap();
            // Add cup to game
            game_1.add_cup(cup);
            game_2.add_cup(cup);
        }
    }

    // Part 1
    let number_runs : u32 = 100;

    game_1.finish_cycle();

    for _ in 0..number_runs { game_1.make_move() }
    let result : Vec<String> = game_1.get_result(1).iter().map(|cup| cup.to_string()).collect();
    println!("Result: '{}'", result.join(""));
    
    // Part 2
    let number_until_cup : Cup = 1000000;
    let number_runs : u32 = 10000000;

    game_2.create_missing_cups_until(number_until_cup);

    for _ in 0..number_runs { game_2.make_move() }
    let mut result : Vec<Cup> = game_2.get_result(1);
    let result_1 : Cup = result.remove(0);
    let result_2 : Cup = result.remove(0);
    println!("Result: '{} x {} = {}'", result_1, result_2, result_1 * result_2);
}
