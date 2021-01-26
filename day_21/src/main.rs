use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

type IgredientName = String;
type Allergen = String;

// ------------------ Information ------------------
struct Information {
    lines : Vec<(HashSet<Allergen>, HashSet<IgredientName>)>,
    possibilities : HashMap<Allergen, HashSet<IgredientName>>,
    igredients : HashSet<IgredientName>,
}

impl Information {
    pub fn new() -> Information {
        Information {
            lines : Vec::new(),
            possibilities : HashMap::new(),
            igredients : HashSet::new(),
        }
    }

    fn add_line(&mut self, igredients : HashSet<IgredientName>, allergens : HashSet<Allergen>) {
        // Adding line
        self.lines.push((igredients.clone(), allergens.clone()));
        // Adding new igredients
        for igredient in igredients.iter() { self.igredients.insert(igredient.clone()); }

        let mut to_update : Vec<(Allergen, HashSet<IgredientName>)> = Vec::new();
        // Check if allergen already addded to possibilities
        for allergen in allergens.into_iter() {
            match self.possibilities.get(&allergen) {
                Some(already_set_igredients) => {
                    let updated_igredients : HashSet<IgredientName> = already_set_igredients.intersection(&igredients)
                        .map(|igredient| igredient.clone()).collect();
                    
                    to_update.push((allergen, updated_igredients));
                },

                None => {
                    self.possibilities.insert(allergen, igredients.clone());
                },
            }
        }

        // Perform updates nbeeded
        for (allergen, igredients) in to_update.into_iter() {
            self.possibilities.insert(allergen, igredients);
        }
    }

    fn solve_down(&mut self) {
        let mut changes : bool = true;
        while changes {
            // Initially no changes
            changes = false;

            // Check which igredients have already a allergen set
            let mut igredients_already_set : HashSet<(Allergen, IgredientName)> = HashSet::new();
            for (allergen, igredients) in self.possibilities.iter() {
                if igredients.len() == 1 {
                    let igredient : String = igredients.iter().next().unwrap().clone();
                    igredients_already_set.insert((allergen.clone(), igredient));
                }
            }

            // Get updated igredients to allergens
            let mut update : HashMap<Allergen, HashSet<IgredientName>> = HashMap::new();
            for (allergen, igredients) in self.possibilities.iter() {
                let mut updated_igredients : HashSet<IgredientName> = igredients.clone();
                for (already_set_allergen, already_set_igredient) in igredients_already_set.iter() {
                    
                    // If it is allergen wich reserved igredient -> pass
                    if already_set_allergen == allergen { continue }
                    // If something removed then set has changed
                    if updated_igredients.remove(already_set_igredient) { changes = true }
                }

                update.insert(allergen.clone(), updated_igredients);
            }

            // Actually update
            self.possibilities = update;
        }
    }

    fn get_igredients_with_no_allergen(&self) -> HashSet<IgredientName> {
        let mut igredients : HashSet<IgredientName> = self.igredients.clone();

        for (_, igredients_with_allergen) in self.possibilities.iter() {
            for igredient_with_allergen in igredients_with_allergen.iter() {
                igredients.remove(igredient_with_allergen);
            }
        }

        return igredients;
    }

    fn get_number_of_times_igredients(&self, igredients : HashSet<IgredientName>) -> u32 {
        let mut count : u32 = 0;

        for (line_igredients, _) in self.lines.iter() {
            for line_igredient in line_igredients.iter() {
                if igredients.contains(line_igredient) {
                    count = count + 1;
                }
            }
        }

        return count;
    }

    fn get_canonical_dangerous_list(&self) -> Vec<IgredientName> {
        let mut canonical_dangerous : Vec<(Allergen, IgredientName)> = Vec::new();
        for (allergen, allergenic_igredients) in self.possibilities.iter() {
            assert_eq!(allergenic_igredients.len(), 1);
            canonical_dangerous.push((allergen.clone(), allergenic_igredients.iter().next().unwrap().clone()));
        }

        canonical_dangerous.sort_by(|a, b| (a.0).partial_cmp(&b.0).unwrap());
        let canonical_dangerous : Vec<IgredientName> = canonical_dangerous.into_iter()
            .map(|set| set.1).collect();

        return canonical_dangerous;
    }
}

// ------------------ Main Code ------------------

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    // Create struct to store all the information
    let mut information : Information = Information::new();
    
    for line in data.iter() {
        let split : Vec<&str> = line.split("(").collect();
        let igredients_string : &str = split[0];
        let allergens_string : &str = &split[1].replace("contains ", "").replace(")", "");

        // Create vector of igredients
        let igredients : HashSet<IgredientName> = igredients_string.split_whitespace()
            .map(|igredient| igredient.to_owned()).collect();
        // Create vector of allergens
        let allergens : HashSet<IgredientName> = allergens_string.split(", ")
            .map(|allergen| allergen.to_owned()).collect();

        information.add_line(igredients, allergens);
    }

    // Solve Part 1
    information.solve_down();
    let igredients_with_no_allergens : HashSet<IgredientName> = information.get_igredients_with_no_allergen();
    let number_times : u32 = information.get_number_of_times_igredients(igredients_with_no_allergens);
    println!("Result {}", number_times);

    // Solve Part 2
    let canonical_dangerous_list : Vec<IgredientName> = information.get_canonical_dangerous_list();
    let canonical_dangerous_string : String = canonical_dangerous_list.join(",");
    println!("Result '{}'", canonical_dangerous_string);
}
