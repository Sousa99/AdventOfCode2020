use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;
use std::fmt;
use std::collections::VecDeque;

// TODO:
// Refactoring and cleanup of code
// Computing orderings could be done in a resonable time if following the same strategy used for counting

type Jolts = i32;

// ----------------- Adapter ------------------
#[derive(Eq, Clone, Debug)]
struct Adapter {
    jolts : Jolts,
    negative_tolerance : Jolts,
}

impl Adapter {
    pub fn new(jolts : Jolts, negative_tolerance : Jolts) -> Adapter {
        Adapter {
            jolts : jolts,
            negative_tolerance : negative_tolerance
        }
    }

    fn get_joltage(&self) -> Jolts { return self.jolts; }

    fn accepts_joltage(&self, joltage : Jolts) -> bool {
        return self.jolts > joltage && self.jolts - self.negative_tolerance <= joltage;
    }
}

impl Ord for Adapter {
    fn cmp(&self, other: &Self) -> Ordering { self.jolts.cmp(&other.jolts) }
}
impl PartialOrd for Adapter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl PartialEq for Adapter {
    fn eq(&self, other: &Self) -> bool { self.jolts == other.jolts }
}


// ----------------- Differential ------------------
struct Differential {
    differential : Jolts,
    count : u32,
}

impl Differential {
    pub fn new(differential : Jolts, count : u32) -> Differential {
        Differential {
            differential : differential,
            count : count,
        }
    }
}

impl fmt::Display for Differential {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Differential of '{}' appears {} time(s)", self.differential, self.count);
    }
}

// ----------------- Possibilities ------------------
#[derive(Debug)]
struct Possibility {
    adapter : Adapter,
    count : u64,
}

impl Possibility {
    pub fn new(adapter : Adapter, count : u64) -> Possibility {
        Possibility {
            adapter : adapter,
            count : count,
        }
    }

    fn get_adapter(&self) -> Adapter { return self.adapter.clone(); }
    fn get_count(&self) -> u64 { return self.count; }
}

// ----------------- Scenario ------------------
struct Scenario {
    device_jolts : Jolts,
    device_negative_tolerance : Jolts,
    outlet_jolts : Jolts,
    adapters : Vec<Adapter>,
    orderings : Vec<Vec<Adapter>>,
    possibilities : Vec<Possibility>
}

impl Scenario {
    pub fn new(outlet_jolts : Jolts) -> Scenario {
        Scenario {
            device_jolts : 0,
            device_negative_tolerance : 3,
            outlet_jolts : outlet_jolts,
            adapters : Vec::new(),
            orderings : Vec::new(),
            possibilities : Vec::new(),
        }
    }

    fn add_adapter(&mut self, jolts : Jolts) {
        let negative_tolerance : Jolts = 3;
        self.adapters.push(Adapter::new(jolts, negative_tolerance));
    }

    fn compute_device_output(&mut self) {
        let device_joltage_addition : Jolts = 3;
        let mut max : Jolts = 0;

        for adapter in self.adapters.iter() {
            let adapter_joltage : Jolts = adapter.get_joltage();
            if adapter_joltage > max { max = adapter_joltage};
        }

        self.device_jolts = max + device_joltage_addition;
    }

    fn order_adapters(&mut self) { self.adapters.sort(); }

    fn compute_differences(&self) -> Result<Vec<Differential>, &'static str> {
        let mut diffs : Vec<Differential> = Vec::new();
        let number_adapters : usize = self.adapters.len();
        
        let error_difference_to_big : &'static str = "There is no possible solution, the gap between two adapters is to big";
        if number_adapters == 0 { return Err("There are no adapters in this scenario"); }

        let mut current_jolts : Jolts = self.outlet_jolts;
        for adapter in self.adapters.iter() {
            if !adapter.accepts_joltage(current_jolts) { return Err(error_difference_to_big); }

            let new_jolts : Jolts = adapter.get_joltage();
            let difference : Jolts = new_jolts - current_jolts;
            current_jolts = new_jolts;

            match diffs.iter_mut().find(|diff| diff.differential == difference) {
                Some(diff) => diff.count = diff.count + 1,
                None => diffs.push(Differential::new(difference, 1)),
            };
        }

        let difference : Jolts = self.device_jolts - current_jolts;
        if difference > self.device_negative_tolerance { return Err(error_difference_to_big); }
        match diffs.iter_mut().find(|diff| diff.differential == difference) {
            Some(diff) => diff.count = diff.count + 1,
            None => diffs.push(Differential::new(difference, 1)),
        };

        return Ok(diffs);
    }

    // Literally compute all possibilities (too exaustive)
    fn compute_orderings(&mut self) -> Vec<Vec<Adapter>> {
        let possibilities : Vec<VecDeque<Adapter>> = self.compute_possibilities_from_point(self.outlet_jolts);

        for possibility in possibilities {
            match possibility.back() {
                Some(last_adapter) => {
                    let difference : Jolts = self.device_jolts - last_adapter.get_joltage();
                    if difference <= self.device_negative_tolerance && difference > 0 {
                        self.orderings.push(Vec::from(possibility));
                    }
                }
                None => ()
            }
        }

        return self.orderings.clone();
    }
    fn compute_possibilities_from_point(&self, value : Jolts) -> Vec<VecDeque<Adapter>> {
        //println!("Dealing with value: {}", value);
        let mut possibilities : Vec<Adapter> = Vec::new();
        for adapter in self.adapters.iter() {
            if adapter.accepts_joltage(value) { possibilities.push(adapter.clone()); }
        }

        let mut available_vectors : Vec<VecDeque<Adapter>> = Vec::new();
        for possibility in possibilities.iter() {

            let mut future_possibilities : Vec<VecDeque<Adapter>> = self.compute_possibilities_from_point(possibility.get_joltage());
            for future_possibility in future_possibilities.iter_mut() {
                future_possibility.push_front(possibility.clone());
            }

            if future_possibilities.len() == 0 {
                let mut temp_vecdeque : VecDeque<Adapter> = VecDeque::new();
                temp_vecdeque.push_back(possibility.clone());
                future_possibilities.push(temp_vecdeque);
            }

            available_vectors.append(&mut future_possibilities);
        }

        return available_vectors;
    }

    // Compute only the number of orderings
    fn compute_number_orderings(&mut self) -> u64 {
        let possibilities : Vec<Possibility> = Vec::new();
        let adapter : Adapter = match self.adapters.first() {
            Some(adapter) => adapter.clone(),
            None => return 0,
        };

        let (possibilities, _) : (Vec<Possibility>, _) = self.compute_number_possibilities_with_adapter(possibilities, adapter);
        self.possibilities = possibilities;

        let mut count : u64 = 0;
        for possibility in self.possibilities.iter() {
            if possibility.get_adapter().accepts_joltage(self.outlet_jolts) {
                count = count + possibility.get_count();
            }
        }

        return count;
    }
    
    fn compute_number_possibilities_with_adapter(&self, mut possibilities : Vec<Possibility>, adapter : Adapter) -> (Vec<Possibility>, u64) {
        let current_jolts : Jolts = adapter.get_joltage();

        // Calculate new possibility
        let mut valid_adapters : Vec<Adapter> = Vec::new();
        for adapter in self.adapters.iter() {
            if adapter.accepts_joltage(current_jolts) { valid_adapters.push(adapter.clone()); }
        }

        let difference_to_device : Jolts = self.device_jolts - current_jolts;
        let mut count : u64 = 0;
        if difference_to_device <= self.device_negative_tolerance && difference_to_device > 0 {count = count + 1; }
        for possible_adapter in valid_adapters.iter() {
            let copy_adapter : Adapter = possible_adapter.clone();
            let already_computed : bool = possibilities.iter().any(|possibility| possibility.get_adapter() == copy_adapter);
            let sub_count : u64;

            if already_computed {
                sub_count = match possibilities.iter().find(|possibility| possibility.get_adapter() == copy_adapter) {
                    Some(possibility) => possibility.get_count(),
                    None => panic!("Should never happen!"),
                };
            } else {
                let result : (Vec<Possibility>, u64) = self.compute_number_possibilities_with_adapter(possibilities, possible_adapter.clone());
                possibilities = result.0;
                sub_count = result.1;
            }

            count = count + sub_count;
        }

        possibilities.push(Possibility::new(adapter, count));
        return (possibilities, count);
    }
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut scenario : Scenario = Scenario::new(0);

    for line in data.iter() {
        let value : u32 = match line.parse() {
            Ok(value) => value,
            Err(e) => panic!("{}", e),
        };

        scenario.add_adapter(value as Jolts);
    }

    scenario.compute_device_output();
    scenario.order_adapters();

    let diffs : Vec<Differential> = match scenario.compute_differences() {
        Ok(diffs) => diffs,
        Err(e) => panic!("{}", e),
    };

    for diff in diffs { println!("{}", diff); }

    /* Too computationally intensive */
    /*
    let orders : Vec<Vec<Adapter>> = scenario.compute_orderings();
    for ordering in orders.iter() { println!("{:?}", ordering); }
    println!("Total number of available orders: {}", orders.len());
    */

    let number_orders : u64 = scenario.compute_number_orderings();
    println!("Total number of available orders: {}", number_orders);
}
