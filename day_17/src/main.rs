use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

// TODO:
// Should not have repetition of code for the different dimensions

type CoordinateUnit = i32;
type Coordinates3D = (CoordinateUnit, CoordinateUnit, CoordinateUnit);
type Coordinates4D = (CoordinateUnit, CoordinateUnit, CoordinateUnit, CoordinateUnit);

// -------------- State --------------
#[derive(Copy, Clone)]
enum State {
    Active,
    Inactive,
}

// -------------- Cube --------------
struct Cube {
    state : State,
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            state : State::Inactive,
        }
    }

    fn get_state(&self) -> State { self.state }
    fn set_state(&mut self, new_state : State) { self.state = new_state }
}

// -------------- 3D World --------------
struct World3D {
    cubes : HashMap<Coordinates3D, Cube>
}

impl World3D {
    pub fn new() -> World3D {
        World3D {
            cubes : HashMap::new(),
        }
    }

    fn add_cube(&mut self, coordinates : Coordinates3D) { self.cubes.insert(coordinates, Cube::new()); }
    fn add_cube_state(&mut self, coordinates : Coordinates3D, state : State) {
        let mut new_cube : Cube = Cube::new();
        new_cube.set_state(state);
        self.cubes.insert(coordinates, new_cube);
    }

    fn get_number_active_cubes(&self) -> u64 {
        let mut count : u64 = 0;
        for cube in self.cubes.iter() {
            match cube.1.get_state() {
                State::Active => count = count + 1,
                _ => (),
            }
        }

        return count;
    }

    fn run_iteration(&mut self) {
        // Check for new cubes to add to world
        let mut coordinates_to_add : HashSet<Coordinates3D> = HashSet::new();
        for world_position in self.cubes.iter() {
            let coordinate : &Coordinates3D = world_position.0;

            for z_variation in -1..2 {
                let z_check : CoordinateUnit = coordinate.2 + z_variation;
                for y_variation in -1..2 {
                    let y_check : CoordinateUnit = coordinate.1 + y_variation;
                    for x_variation in -1..2 {
                        if x_variation == 0 && y_variation == 0 && z_variation == 0 { continue }

                        let x_check : CoordinateUnit = coordinate.0 + x_variation;
                        let check_coordinates : Coordinates3D = (x_check, y_check, z_check);
                        if !self.cubes.contains_key(&check_coordinates) {
                            coordinates_to_add.insert(check_coordinates);
                        }
                    }
                }
            }
        }

        // Add new cubes
        for coordinate in coordinates_to_add.into_iter() { self.add_cube(coordinate) }

        // Check cubes that need updates
        let mut updates : HashMap<Coordinates3D, State> = HashMap::new();
        for world_position in self.cubes.iter() {
            let coordinate : &Coordinates3D = world_position.0;
            let cube : &Cube = world_position.1;
            let cube_state : State = cube.get_state();

            let mut count_active_nearby : u64 = 0;

            for z_variation in -1..2 {
                let z_check : CoordinateUnit = coordinate.2 + z_variation;
                for y_variation in -1..2 {
                    let y_check : CoordinateUnit = coordinate.1 + y_variation;
                    for x_variation in -1..2 {
                        if x_variation == 0 && y_variation == 0 && z_variation == 0 { continue }

                        let x_check : CoordinateUnit = coordinate.0 + x_variation;
                        let check_coordinates : Coordinates3D = (x_check, y_check, z_check);
                        
                        let nearby_cube : Option<&Cube> = self.cubes.get(&check_coordinates);
                        match nearby_cube {
                            Some(nearby_cube) =>{
                                let nearby_cube_state : State = nearby_cube.get_state();
                                match nearby_cube_state {
                                    State::Active => count_active_nearby = count_active_nearby + 1,
                                    _ => ()
                                }
                            },
                            None => ()
                        }
                    }
                }
            }

            match cube_state {
                State::Active if (count_active_nearby != 2) && (count_active_nearby != 3) => {
                    updates.insert(coordinate.clone(), State::Inactive);
                },
                State::Inactive if count_active_nearby == 3 => {
                    updates.insert(coordinate.clone(), State::Active);
                }
                _ => ()
            }
        }

        // Update Cubes
        for update in updates.into_iter() {
            let coordinate : Coordinates3D = update.0;
            let next_state : State = update.1;
            self.cubes.get_mut(&coordinate).unwrap().set_state(next_state);
        }
    }
}

// -------------- 4D World --------------
struct World4D {
    cubes : HashMap<Coordinates4D, Cube>
}

impl World4D {
    pub fn new() -> World4D {
        World4D {
            cubes : HashMap::new(),
        }
    }

    fn add_cube(&mut self, coordinates : Coordinates4D) { self.cubes.insert(coordinates, Cube::new()); }
    fn add_cube_state(&mut self, coordinates : Coordinates4D, state : State) {
        let mut new_cube : Cube = Cube::new();
        new_cube.set_state(state);
        self.cubes.insert(coordinates, new_cube);
    }

    fn get_number_active_cubes(&self) -> u64 {
        let mut count : u64 = 0;
        for cube in self.cubes.iter() {
            match cube.1.get_state() {
                State::Active => count = count + 1,
                _ => (),
            }
        }

        return count;
    }

    fn run_iteration(&mut self) {
        // Check for new cubes to add to world
        let mut coordinates_to_add : HashSet<Coordinates4D> = HashSet::new();
        for world_position in self.cubes.iter() {
            let coordinate : &Coordinates4D = world_position.0;

            for w_variation in -1..2 {
                let w_check : CoordinateUnit = coordinate.3 + w_variation;
                for z_variation in -1..2 {
                    let z_check : CoordinateUnit = coordinate.2 + z_variation;
                    for y_variation in -1..2 {
                        let y_check : CoordinateUnit = coordinate.1 + y_variation;
                        for x_variation in -1..2 {
                            if x_variation == 0 && y_variation == 0 && z_variation == 0 && w_variation == 0 { continue }

                            let x_check : CoordinateUnit = coordinate.0 + x_variation;
                            let check_coordinates : Coordinates4D = (x_check, y_check, z_check, w_check);
                            if !self.cubes.contains_key(&check_coordinates) {
                                coordinates_to_add.insert(check_coordinates);
                            }
                        }
                    }
                }
            }
        }

        // Add new cubes
        for coordinate in coordinates_to_add.into_iter() { self.add_cube(coordinate) }

        // Check cubes that need updates
        let mut updates : HashMap<Coordinates4D, State> = HashMap::new();
        for world_position in self.cubes.iter() {
            let coordinate : &Coordinates4D = world_position.0;
            let cube : &Cube = world_position.1;
            let cube_state : State = cube.get_state();

            let mut count_active_nearby : u64 = 0;

            for w_variation in -1..2 {
                let w_check : CoordinateUnit = coordinate.3 + w_variation;
                for z_variation in -1..2 {
                    let z_check : CoordinateUnit = coordinate.2 + z_variation;
                    for y_variation in -1..2 {
                        let y_check : CoordinateUnit = coordinate.1 + y_variation;
                        for x_variation in -1..2 {
                            if x_variation == 0 && y_variation == 0 && z_variation == 0 && w_variation == 0 { continue }

                            let x_check : CoordinateUnit = coordinate.0 + x_variation;
                            let check_coordinates : Coordinates4D = (x_check, y_check, z_check, w_check);
                            
                            let nearby_cube : Option<&Cube> = self.cubes.get(&check_coordinates);
                            match nearby_cube {
                                Some(nearby_cube) =>{
                                    let nearby_cube_state : State = nearby_cube.get_state();
                                    match nearby_cube_state {
                                        State::Active => count_active_nearby = count_active_nearby + 1,
                                        _ => ()
                                    }
                                },
                                None => ()
                            }
                        }
                    }
                }
            }

            match cube_state {
                State::Active if (count_active_nearby != 2) && (count_active_nearby != 3) => {
                    updates.insert(coordinate.clone(), State::Inactive);
                },
                State::Inactive if count_active_nearby == 3 => {
                    updates.insert(coordinate.clone(), State::Active);
                }
                _ => ()
            }
        }

        // Update Cubes
        for update in updates.into_iter() {
            let coordinate : Coordinates4D = update.0;
            let next_state : State = update.1;
            self.cubes.get_mut(&coordinate).unwrap().set_state(next_state);
        }
    }
}

fn get_state_from_code(characther : char) -> State {
    match characther {
        '#' => State::Active,
        '.' => State::Inactive,
        _ => panic!("Not a valid code for a state!"),
    }
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut world_3d : World3D = World3D::new();
    let mut world_4d : World4D = World4D::new();

    let w : CoordinateUnit = 0;
    let z : CoordinateUnit = 0;
    for (line_index, line) in data.iter().enumerate() {
        let y : CoordinateUnit = line_index as CoordinateUnit;

        for (char_index, characther) in line.chars().enumerate() {
            let x : CoordinateUnit = char_index as CoordinateUnit;
            let coordinates_3d : Coordinates3D = (x, y, z);
            let coordinates_4d : Coordinates4D = (x, y, z, w);
            let state : State = get_state_from_code(characther);

            world_3d.add_cube_state(coordinates_3d, state);
            world_4d.add_cube_state(coordinates_4d, state);
        }
    }

    let boot_sequence : usize = 6;
    for _ in 0..boot_sequence { world_3d.run_iteration() }
    for _ in 0..boot_sequence { world_4d.run_iteration() }
    println!("At the end of the '{}' cycles there are '{}' active cubes in a 3D World.", boot_sequence, world_3d.get_number_active_cubes());
    println!("At the end of the '{}' cycles there are '{}' active cubes in a 4D World.", boot_sequence, world_4d.get_number_active_cubes());
}
