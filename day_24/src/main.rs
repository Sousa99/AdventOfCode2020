use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::ops::Add;

const DEBUG : bool = false;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

// ------------------- CoordinateUnit -------------------
#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct CoordinateUnit {
    integral: i64,
    fractional: u64
}

impl CoordinateUnit {
    pub fn new(number : f32) -> CoordinateUnit {
        CoordinateUnit {
            integral : number.floor() as i64,
            fractional : ((number - number.floor()) * 1000.0).round() as u64,
        }
    }
}

impl Add for CoordinateUnit {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let integral_only : i64 = self.integral + other.integral;
        let fractional_only : u64 = self.fractional + other.fractional;
        let fractional_surpass : u64 = fractional_only / 1000;

        Self {
            integral: integral_only + fractional_surpass as i64,
            fractional: fractional_only - fractional_surpass * 1000,
        }
    }
}

type Coordinates = (CoordinateUnit, CoordinateUnit);

// ------------------- Direction -------------------
enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast
}

// ------------------- Tile -------------------
#[derive(Copy, Clone)]
enum Tile {
    White,
    Black,
}

// ------------------- Tile Floor -------------------
struct TileFloor {
    unit : f32,
    tiles : HashMap<Coordinates, Tile>,
}

impl TileFloor {
    pub fn new() -> TileFloor {
        TileFloor {
            unit : 1.0,
            tiles : hashmap!((CoordinateUnit::new(0.0), (CoordinateUnit::new(0.0))) => Tile::White)
        }
    }

    fn get_coordinates(&self, directions : Vec<Direction>) -> Coordinates {
        let spacing : f32 = self.unit * 2.0;
        let mut coordinates : Coordinates = (CoordinateUnit::new(0.0), (CoordinateUnit::new(0.0)));

        for direction in directions.into_iter() {
            let angle : f32 = match direction {
                Direction::Southeast => 300.0,
                Direction::Southwest => 240.0,
                Direction::West => 180.0,
                Direction::Northwest => 120.0,
                Direction::Northeast => 60.0,
                Direction::East => 0.0,
            };
            
            let angle : f32 = angle.to_radians();
            let tmp_coordinates : Coordinates = (CoordinateUnit::new(spacing * angle.cos()), CoordinateUnit::new(spacing * angle.sin()));
            coordinates = (coordinates.0 + tmp_coordinates.0, coordinates.1 + tmp_coordinates.1);
        }

        return coordinates;
    }

    fn flip(&mut self, coordinates : Coordinates) {
        match self.tiles.get(&coordinates) {
            Some(tile) => {
                let new_tile : Tile = match tile {
                    Tile::White => Tile::Black,
                    Tile::Black => Tile::White,
                };

                self.tiles.insert(coordinates, new_tile);
            },
            None => { self.tiles.insert(coordinates, Tile::Black); },
        }
    }

    fn get_number_black_tiles(&self) -> u32 {
        let mut count : u32 = 0;

        for (_, tile) in self.tiles.iter() {
            match tile {
                Tile::Black => count = count + 1,
                _ => (),
            }
        }

        return count;
    }

    fn create_white_tiles_surround(&mut self) {
        let spacing : f32 = self.unit * 2.0;

        // Figure out which coordinates need to be added
        let mut coordinates_to_add : Vec<Coordinates> = Vec::new();
        for (&coordinates, &tile) in self.tiles.iter() {
            if matches!(tile, Tile::White) { continue }

            for angle in (0..360).step_by(60) {
                let x : f32 = spacing * (angle as f32).to_radians().cos();
                let y : f32 = spacing * (angle as f32).to_radians().sin();

                let tmp_coordinates : Coordinates = (CoordinateUnit::new(x), CoordinateUnit::new(y));
                let tmp_coordinates : Coordinates = (coordinates.0 + tmp_coordinates.0, coordinates.1 + tmp_coordinates.1);

                if !self.tiles.contains_key(&tmp_coordinates) { coordinates_to_add.push(tmp_coordinates) }
            }
        }

        // Actually add coordinates
        for coordinate in coordinates_to_add.into_iter() { self.tiles.insert(coordinate, Tile::White); }
    }

    fn make_move(&mut self) {
        let spacing : f32 = self.unit * 2.0;
        self.create_white_tiles_surround();

        let mut coordinates_to_flip : Vec<Coordinates> = Vec::new();
        // Iterate over to figure out which to flip
        for (&coordinates, &tile) in self.tiles.iter() {
            let mut count_black : u32 = 0;

            for angle in (0..360).step_by(60) {
                let x : f32 = spacing * (angle as f32).to_radians().cos();
                let y : f32 = spacing * (angle as f32).to_radians().sin();

                let tmp_coordinates : Coordinates = (CoordinateUnit::new(x), CoordinateUnit::new(y));
                let tmp_coordinates : Coordinates = (coordinates.0 + tmp_coordinates.0, coordinates.1 + tmp_coordinates.1);

                match self.tiles.get(&tmp_coordinates) {
                    Some(next_tile) => {
                        match next_tile {
                            Tile::White => (),
                            Tile::Black => count_black = count_black + 1,
                        }
                    },
                    None => (),
                }
            }

            match tile {
                Tile::Black if count_black == 0 || count_black > 2 => coordinates_to_flip.push(coordinates),
                Tile::White if count_black == 2 => coordinates_to_flip.push(coordinates),
                _ => (),
            }
        }

        // Actually flip them
        for coordinates in coordinates_to_flip { self.flip(coordinates) }
    }
}

// ------------------- Aux Function -------------------
fn get_direction_to_char(element : String) -> Option<Direction> {
    match element.as_str() {
        "e" => Some(Direction::East),
        "se" => Some(Direction::Southeast),
        "sw" => Some(Direction::Southwest),
        "w" => Some(Direction::West),
        "nw" => Some(Direction::Northwest),
        "ne" => Some(Direction::Northeast),
        _ => None,
    }
}

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut floor : TileFloor = TileFloor::new();
    for line in data.iter() {
        let mut directions : Vec<Direction> = Vec::new();
        let mut current_string : String = String::new();

        for element in line.chars() {
            current_string.push(element);
            match get_direction_to_char(current_string.clone()) {
                Some(direction) => {
                    directions.push(direction);
                    current_string = String::new();
                },

                None => ()
            }
        }

        let coordinates : Coordinates = floor.get_coordinates(directions);
        floor.flip(coordinates);
    }

    // Part 1
    println!("Result: '{}' (Part 1)", floor.get_number_black_tiles());
    println!("------------------------------------------");

    // Part 2
    let number_days : u32 = 100;
    for day in 0..number_days {
        floor.make_move();
        if DEBUG { println!("Day {}: {}", day + 1, floor.get_number_black_tiles()) }
    }
    println!("Result: '{}' (Part 2)", floor.get_number_black_tiles());
}
