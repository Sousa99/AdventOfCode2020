use std::fs::File;
use std::io::{BufRead, BufReader};

// TODO:
// Find if there is a way to find the solution if the bus ids are not relatively prime to each other

type BusNumber = u32;
type Time = u64;

// ---------------- Bus ----------------
struct Bus {
    id : Option<BusNumber>,
}

impl Bus {
    pub fn new(id : Option<BusNumber>) -> Bus {
        Bus {
            id : id,
        }
    }

    fn get_bus_id(&self) -> Option<BusNumber> { self.id }
}

// ---------------- Scenario ----------------
struct Scenario {
    buses : Vec<Bus>,
    passenger_time : Time,
    earliest_time : Vec<Option<Time>>,
    maximum : Option<(usize, BusNumber)>
}

impl Scenario {
    pub fn new(passenger_time : Time) -> Scenario {
        Scenario {
            buses : Vec::new(),
            passenger_time : passenger_time,
            earliest_time : Vec::new(),
            maximum : None
        }
    }

    fn get_passenger_time(&self) -> Time { self.passenger_time }

    fn add_bus(&mut self, bus_id : Option<BusNumber>) {
        let add_index : usize = self.buses.len();
        match (self.maximum, bus_id) {
            (None, Some(id)) => self.maximum = Some((add_index, id)),
            (Some(current_maximum), Some(id)) if id > current_maximum.1 => self.maximum = Some((add_index, id)),
            (_, _) => (),
        }

        self.buses.push(Bus::new(bus_id));
    }

    fn compute_earliest_times(&mut self) {
        self.earliest_time = Vec::new();
        for bus in self.buses.iter() {
            let bus_id : Option<BusNumber> = bus.get_bus_id();
            let time : Option<Time>  = match bus_id {
                Some(id) => Some(((self.passenger_time as f64  / id as f64).ceil() * (id as f64)) as Time),
                None => None
            };
            
            self.earliest_time.push(time);
        }
    }

    fn get_bus_minimum(&self) -> Result<(BusNumber, Time), &'static str> {
        let mut minimum : Option<(BusNumber, Time)> = None;

        for (bus, earliest_time) in self.buses.iter().zip(self.earliest_time.iter()) {
            match (bus.get_bus_id(), *earliest_time) {
                (Some(id), Some(time)) => {
                    match minimum {
                        Some(current_min) if current_min.1 > time => minimum = Some((id, time)),
                        None => minimum = Some((id, time)),
                        _ => continue,
                    }
                }
                _ => continue,
            }
        }

        match minimum {
            Some(info) => return Ok(info),
            None => return Err("All buses are non-specified!"),
        }
    }

    fn compute_sequential_earliest_point(&self) -> Time {
        let mut calculation : Calculation = Calculation::new();

        for (index, bus) in self.buses.iter().enumerate() {
            let bus_id : Option<BusNumber> = bus.get_bus_id();
            match bus_id {
                Some(id) => {
                    let mut remainder : FormulaValue = - (index as FormulaValue);
                    while remainder < 0 { remainder = remainder + id as FormulaValue; }

                    calculation.add_equation(id as FormulaValue, remainder)
                },
                None => (),
            };
        }

        return calculation.calculate() as Time;
    }
}

// ---------------- Calculation (Chinese Remainder Theorem) ----------------
type FormulaValue = i64;
struct EquationStep {
    m : FormulaValue,
    a : FormulaValue,
    z : Option<FormulaValue>,
    y : Option<FormulaValue>,
    w : Option<FormulaValue>,
}

impl EquationStep {
    pub fn new(m : FormulaValue, a : FormulaValue) -> EquationStep {
        EquationStep {
            m : m,
            a : a,
            z : None,
            y : None,
            w : None,
        }
    }

    fn calculate_z(&mut self, module : FormulaValue) { self.z = Some(module / self.m); }
    fn calculate_y(&mut self) {
        match self.z {
            Some(z) => {
                // Euclid's finding of inverses
                let z_moduled : FormulaValue = z % self.m;
                let mut y : FormulaValue = 1;

                loop {
                    if (z_moduled * y) % self.m == 1 { break; }
                    y = y + 1;
                }

                self.y = Some(y);

            },
            _ => panic!("First please define 'z'."),
        }
    }
    fn calculate_w(&mut self, module : FormulaValue) {
        match (self.z, self.y) {
            (Some(z), Some(y)) => self.w = Some((z * y) % module),
            _ => panic!("First please define 'z' and 'y'."),
        }
    }
}

struct Calculation {
    m : FormulaValue,
    equations : Vec<EquationStep>,
}

impl Calculation {
    pub fn new() -> Calculation {
        Calculation {
            m : 1,
            equations : Vec::new(),
        }
    }

    fn add_check_module(&self, module : FormulaValue) -> bool {
        for equation in self.equations.iter() {
            // Find gcd between this new module and already added
            let minimum : FormulaValue = std::cmp::min(equation.m, module);
            for value in 2..minimum {
                if equation.m % value == 0 && module % value == 0 {
                    return false;
                }
            }
        }

        return true;
    }

    fn add_equation(&mut self, module : FormulaValue, remainder : FormulaValue) {
        if !self.add_check_module(module) {
            panic!("This approach using Chinese Remainder Formula only works with modules relatively prime!");
        }

        println!("Added equation: x ≡ {} (mod {})", remainder % module, module);

        self.equations.push(EquationStep::new(module, remainder % module));
        self.m = self.m * module;
    }

    fn calculate(&mut self) -> FormulaValue {
        for equation in self.equations.iter_mut() {
            equation.calculate_z(self.m);
            equation.calculate_y();
            equation.calculate_w(self.m);
        }

        let mut result : FormulaValue = 0;
        for equation in self.equations.iter() {
            match equation.w {
                Some(w) => result = result + equation.a * w,
                _ => panic!("First please define 'w'."),
            }
        }

        return result % self.m;
    }
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    // Create Scenario and parse passenger time
    let passenger_time : Time = match data[0].parse() {
        Ok(value) => value,
        Err(e) => panic!("{}", e),
    };
    let mut scenario : Scenario = Scenario::new(passenger_time);

    // Parse Buses Id's
    let split : std::str::Split<&str> = data[1].split(",");
    for code in split {
        let mut bus_id : Option<BusNumber> = None;
        if code != "x" {
            bus_id = match code.parse() {
                Ok(value) => Some(value),
                Err(e) => panic!("{}", e),
            };
        }

        scenario.add_bus(bus_id);
    }
    
    // Compute earliest_time (Part 1)
    scenario.compute_earliest_times();
    match scenario.get_bus_minimum() {
        Ok((bus_id, earliest_time)) => {
            let waiting_time : Time = earliest_time - scenario.get_passenger_time();
            println!("Bus ID{} is the first to arrive at {} minutes.", bus_id, earliest_time);
            println!("This is {} minutes after passenger.", waiting_time);
            println!("Result: {}", bus_id as Time * waiting_time);
        }
        Err(e) => println!("{}", e),
    };

    println!("-------------------------------");
    // Compute first t following rules (Part 2)
    let t : Time = scenario.compute_sequential_earliest_point();
    println!("-------------------------------");
    println!("Valid timestamp that follows the sequence: {}", t);

}
