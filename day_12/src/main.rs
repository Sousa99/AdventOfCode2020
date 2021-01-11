use std::fs::File;
use std::io::{BufRead, BufReader};

type CoordinateUnit = f32;
type Coordinates = (CoordinateUnit, CoordinateUnit);

// --------------------- Operation ---------------------
#[derive(Copy, Clone)]
enum Operation {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,

    RotateLeft,
    RotateRight,
    MoveForward,
}

// --------------------- Ferry ---------------------
struct Ferry {
    // Assuming East is direction 0
    position : Coordinates,
    direction : CoordinateUnit
}

impl Ferry {
    pub fn new() -> Ferry {
        Ferry {
            position : (0.0, 0.0),
            direction : 0.0
        }
    }

    fn get_position(&self) -> Coordinates { self.position }

    fn move_north(&mut self, value : CoordinateUnit) { self.position.1 = self.position.1 + value }
    fn move_south(&mut self, value : CoordinateUnit) { self.position.1 = self.position.1 - value }
    fn move_east(&mut self, value : CoordinateUnit) { self.position.0 = self.position.0 + value }
    fn move_west(&mut self, value : CoordinateUnit) { self.position.0 = self.position.0 - value }

    fn rotate_left(&mut self, value : CoordinateUnit) { self.direction = self.direction + value }
    fn rotate_right(&mut self, value : CoordinateUnit) { self.direction = self.direction - value }
    fn move_forward(&mut self, value : CoordinateUnit) {
        self.position.0 = self.position.0 + value * self.direction.to_radians().cos();
        self.position.1 = self.position.1 + value * self.direction.to_radians().sin();
    }

    fn run_operation(&mut self, operation : Operation, value : CoordinateUnit) {
        match operation {
            Operation::MoveNorth => self.move_north(value),
            Operation::MoveSouth => self.move_south(value),
            Operation::MoveEast => self.move_east(value),
            Operation::MoveWest => self.move_west(value),

            Operation::RotateLeft => self.rotate_left(value),
            Operation::RotateRight => self.rotate_right(value),
            Operation::MoveForward => self.move_forward(value),
        }
    }
}

fn get_operation_to_code(code : &str) -> Operation {
    let operation : Operation = match code {
        "N" => Operation::MoveNorth,
        "S" => Operation::MoveSouth,
        "E" => Operation::MoveEast,
        "W" => Operation::MoveWest,

        "L" => Operation::RotateLeft,
        "R" => Operation::RotateRight,
        "F" => Operation::MoveForward,
        _ => panic!("Option not recognized!")
    };

    return operation;
}
// --------------------- Waypoint ---------------------
struct Waypoint {
    // Assuming East is direction 0
    position : Coordinates,
    waypoint : Coordinates,
}

impl Waypoint {
    pub fn new() -> Waypoint {
        Waypoint {
            position : (0.0, 0.0),
            waypoint : (10.0, 1.0),
        }
    }

    fn get_position(&self) -> Coordinates { self.position }

    fn move_north(&mut self, value : CoordinateUnit) { self.waypoint.1 = self.waypoint.1 + value }
    fn move_south(&mut self, value : CoordinateUnit) { self.waypoint.1 = self.waypoint.1 - value }
    fn move_east(&mut self, value : CoordinateUnit) { self.waypoint.0 = self.waypoint.0 + value }
    fn move_west(&mut self, value : CoordinateUnit) { self.waypoint.0 = self.waypoint.0 - value }

    fn rotate_left(&mut self, value : CoordinateUnit) {
        let east_vector : Coordinates = (1.0, 0.0);

        let angle : CoordinateUnit = (self.waypoint.1.atan2(self.waypoint.0) - east_vector.1.atan2(east_vector.0)).to_degrees();
        let radius : CoordinateUnit = (self.waypoint.0.powf(2.0) + self.waypoint.1.powf(2.0)).sqrt();
        
        let angle : CoordinateUnit = angle + value;
        self.waypoint = (radius * angle.to_radians().cos(), radius * angle.to_radians().sin());
    }
    fn rotate_right(&mut self, value : CoordinateUnit) {
        let east_vector : Coordinates = (1.0, 0.0);

        let angle : CoordinateUnit = (self.waypoint.1.atan2(self.waypoint.0) - east_vector.1.atan2(east_vector.0)).to_degrees();
        let radius : CoordinateUnit = (self.waypoint.0.powf(2.0) + self.waypoint.1.powf(2.0)).sqrt();
        
        let angle : CoordinateUnit = angle - value;
        self.waypoint = (radius * angle.to_radians().cos(), radius * angle.to_radians().sin());
    }
    fn move_forward(&mut self, value : CoordinateUnit) {
        self.position.0 = self.position.0 + value * self.waypoint.0;
        self.position.1 = self.position.1 + value * self.waypoint.1;
    }

    fn run_operation(&mut self, operation : Operation, value : CoordinateUnit) {
        match operation {
            Operation::MoveNorth => self.move_north(value),
            Operation::MoveSouth => self.move_south(value),
            Operation::MoveEast => self.move_east(value),
            Operation::MoveWest => self.move_west(value),

            Operation::RotateLeft => self.rotate_left(value),
            Operation::RotateRight => self.rotate_right(value),
            Operation::MoveForward => self.move_forward(value),
        }
    }
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut ferry : Ferry = Ferry::new();
    let mut waypoint : Waypoint = Waypoint::new();
    
    for line in data.iter() {
        let operation : &str = &line[..1];
        let value : &str = &line[1..];

        let operation : Operation = get_operation_to_code(operation);
        let value : CoordinateUnit = match value.parse() {
            Ok(value) => value,
            Err(e) => panic!("{}", e),
        };

        // Update Ferry (Part 1)
        ferry.run_operation(operation, value);
        // Update Waypoint (Part 2)
        waypoint.run_operation(operation, value);
    }

    let current_position : Coordinates = ferry.get_position();
    println!("Current ferry position (part 1): ({}, {})", current_position.0, current_position.1);

    let current_position : Coordinates = waypoint.get_position();
    println!("Current ferry position (part 2): ({}, {})", current_position.0, current_position.1);
}
