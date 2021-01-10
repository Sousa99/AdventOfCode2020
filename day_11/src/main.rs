use std::fs::File;
use std::io::{BufRead, BufReader};

type SeatNumber = i32;

// ------------------ Rules for Challenge ------------------
#[derive(Copy, Clone)]
enum Rule {
    ImmediatelyNext,
    FirstVisible,
}

// ------------------ Seat State ------------------
#[derive(Copy, Clone)]
enum SeatState {
    Floor,
    Free,
    Occupied,
}

fn get_code_to_state(state : SeatState) -> String {
    let code = match state {
        SeatState::Floor => ".",
        SeatState::Free => "L",
        SeatState::Occupied => "#"
    };

    return code.to_string();
}

fn get_state_to_code(code : String) -> SeatState {
    if code == "." { return SeatState::Floor; }
    else if code == "L" { return SeatState::Free; }
    else if code == "#" { return SeatState::Occupied; }
    else { panic!("Code not recognized!"); }
}

// ------------------ Seat ------------------
#[derive(Clone, Copy)]
struct Seat {
    state : SeatState,
    row : SeatNumber,
    collumn : SeatNumber
}

impl Seat {
    pub fn new(state : SeatState, row : SeatNumber, collumn : SeatNumber) -> Seat {
        Seat {
            state : state,
            row : row,
            collumn : collumn,
        }
    }

    fn get_state(&self) -> SeatState { self.state.clone() }
    fn set_state(&mut self, new_state : SeatState) { self.state = new_state; }
    fn get_row(&self) -> SeatNumber { self.row }
    fn get_collumn(&self) -> SeatNumber { self.collumn }

    fn get_seat_code(&self) -> String {
        return get_code_to_state(self.state.clone());
    }
}

// ------------------ Scenario ------------------
struct Scenario {
    current_row : SeatNumber,
    current_collumn : SeatNumber,
    seats : Vec<Vec<Seat>>
}

impl Scenario {
    pub fn new() -> Scenario {
        Scenario {
            seats : Vec::new(),
            current_row : -1,
            current_collumn : -1
        }
    }

    fn add_row(&mut self) {
        self.current_row = self.current_row + 1;
        self.current_collumn = -1;
        self.seats.push(Vec::new());
    }

    fn add_seat(&mut self, code : String) { 
        self.current_collumn = self.current_collumn + 1;
        let new_seat_state : SeatState = get_state_to_code(code);

        match self.seats.last_mut() {
            Some(row) => row.push(Seat::new(new_seat_state, self.current_row, self.current_collumn)),
            None => panic!("This should only be called after adding at least one row!"),
        }
    }

    fn reset_seats_to_free(&mut self) {
        for row in self.seats.iter_mut() {
            for seat in row.iter_mut() {
                match seat.get_state() {
                    SeatState::Occupied => seat.set_state(SeatState::Free),
                    _ => (),
                }
            }
        }
    }

    fn print_scenario(&self) {
        for row in self.seats.iter() {
            let mut row_string : String = String::new();
            for seat in row.iter() { row_string.push_str(&seat.get_seat_code()); }
            println!("{}", row_string);
        }
    }

    fn get_number_occupied_seats(&self) -> u32 {
        let mut counter : u32 = 0;
        for row in self.seats.iter() {
            for seat in row.iter() {
                match seat.get_state() {
                    SeatState::Occupied => counter = counter + 1,
                    _ => (),
                }
            }
        }

        return counter;
    }

    fn get_seat_at(&self, row_number : SeatNumber, collumn_number : SeatNumber) -> Option<&Seat> {
        let number_rows : SeatNumber = self.seats.len() as SeatNumber;
        if row_number < 0 || row_number >= number_rows { return None; }
        
        let row : &Vec<Seat> = &self.seats[row_number as usize];
        let number_collumns : SeatNumber = row.len() as SeatNumber;
        if collumn_number < 0 || collumn_number >= number_collumns { return None; }
    
        return Some(&self.seats[row_number as usize][collumn_number as usize]);
    }

    fn compute_number_occupied_by_rule(&self, current_row : SeatNumber, current_collumn : SeatNumber, rule : Rule) -> u32 {
        let mut count_occupied : u32 = 0;

        match rule {
            Rule::ImmediatelyNext => {
                for index_row in -1..2 {
                    for index_collumn in -1..2 {
                        if index_row == 0 && index_collumn == 0 { continue; }
                        let next_to_seat : Option<&Seat> = self.get_seat_at(current_row + index_row, current_collumn + index_collumn);

                        match next_to_seat {
                            Some(next_to_seat) => {
                                match next_to_seat.get_state() {
                                    SeatState::Occupied => count_occupied = count_occupied + 1,
                                    _ => ()
                                }
                            },
                            None => ()
                        }
                    }
                }
            },

            Rule::FirstVisible => {
                for permutation_row in -1..2 {
                    for permutation_collumn in -1..2 {
                        if permutation_row == 0 && permutation_collumn == 0 { continue; }

                        let permutation : (SeatNumber, SeatNumber) = (permutation_row, permutation_collumn);
                        let mut at_point : (SeatNumber, SeatNumber) = (current_row, current_collumn);

                        loop  {
                            at_point = (at_point.0 + permutation.0, at_point.1 + permutation.1);
                            let next_to_seat : Option<&Seat> = self.get_seat_at(at_point.0, at_point.1);

                            match next_to_seat {
                                Some(next_to_seat) => {
                                    match next_to_seat.get_state() {
                                        SeatState::Occupied => {
                                            count_occupied = count_occupied + 1;
                                            break;
                                        },
                                        SeatState::Free => break,
                                        SeatState::Floor => ()
                                    }
                                },
                                None => break
                            }
                        }
                    }
                }
            }
        }

        return count_occupied;
    }

    fn run_iteration(&mut self, rule : Rule) -> Result<&'static str, &'static str> {
        let mut none_changed : bool = true;
        let mut next_states : Vec<Vec<Option<SeatState>>> = Vec::new();

        // Compute next states
        for row in self.seats.iter() {
            let mut row_next_states : Vec<Option<SeatState>> = Vec::new();

            for seat in row.iter() {
                let current_row : SeatNumber = seat.get_row();
                let current_collumn : SeatNumber = seat.get_collumn();

                let count_occupied : u32 = self.compute_number_occupied_by_rule(current_row, current_collumn, rule);

                let next_state : Option<SeatState> = match seat.get_state() {
                    SeatState::Free if count_occupied == 0 => Some(SeatState::Occupied),
                    SeatState::Free => None,
                    SeatState::Occupied => {
                        match rule {
                            Rule::ImmediatelyNext if count_occupied >= 4 => Some(SeatState::Free),
                            Rule::FirstVisible if count_occupied >= 5 => Some(SeatState::Free),
                            _ => None,
                        }
                    },
                    SeatState::Floor => None,
                };

                match next_state {
                    Some(_) => none_changed = false,
                    None => (),
                }

                row_next_states.push(next_state);
            }

            next_states.push(row_next_states);
        }

        // Update Seat States
        for row_iter in self.seats.iter_mut().zip(next_states.into_iter()) {
            let (row_current, row_next) = row_iter;

            for seat_iter in row_current.iter_mut().zip(row_next.into_iter()) {
                let (seat_current, seat_next) : (&mut Seat, Option<SeatState>) = seat_iter;

                match seat_next {
                    Some(next_state) => seat_current.set_state(next_state),
                    None => (),
                }
            }
        }

        if none_changed { return Ok("Nothing changed!"); }
        return Ok("Iteration Done!")
    }
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut scenario : Scenario = Scenario::new();
    for line in data.iter() {
        scenario.add_row();

        for characther in line.chars() {
            scenario.add_seat(characther.to_string());
        }
    }

    //scenario.print_scenario();
    println!("At the beggining there are '{}' occupied seats.", scenario.get_number_occupied_seats());
    
    // --------------------- Part 1 ---------------------
    loop {
        match scenario.run_iteration(Rule::ImmediatelyNext) {
            Ok("Nothing changed!") => break,
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };
    }
    //scenario.print_scenario();
    println!("At the end there are '{}' occupied seats.", scenario.get_number_occupied_seats());

    scenario.reset_seats_to_free();

    // --------------------- Part 2 ---------------------
    loop {
        match scenario.run_iteration(Rule::FirstVisible) {
            Ok("Nothing changed!") => break,
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };
    }
    //scenario.print_scenario();
    println!("At the end there are '{}' occupied seats.", scenario.get_number_occupied_seats());

}
