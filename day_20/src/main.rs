use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use rand::Rng;

// TODO:
// First of all it takes quite a bit to solve the puzzle
// and the process of solving uses random to remove x percent of the tiles
// if it got stuck or if it surpassed the limits
// Maybe there is a good solution that will solve both of the problems at the same tim

// ------------ Processing Phase -----------------
enum ProcessingPhase {
    GetPuzzleNumber,
    GetPuzzle,
}

// ------------ Puzzle Piece Side -----------------
#[derive(Copy, Clone)]
enum PuzzlePieceSide {
    Right,
    Bottom,
    Left,
    Top
}

// ------------ Puzzle Piece -----------------
#[derive(Clone)]
struct PuzzlePiece {
    piece_number : u64,
    pieces : HashMap<(u64, u64), char>,
    max_row : u64,
    max_column : u64,
}

impl PuzzlePiece {
    pub fn new(piece_number : u64) -> PuzzlePiece {
        PuzzlePiece {
            piece_number : piece_number,
            pieces : HashMap::new(),
            max_row : 0,
            max_column : 0,
        }
    }

    fn get_piece_number(&self) -> u64 { self.piece_number }
    fn get_size(&self) -> u64 {
        assert_eq!(self.max_row, self.max_column);
        return self.max_row;
    }

    fn add_element(&mut self, row : u64, column : u64, element : char) {
        self.pieces.insert((row, column), element);
        if row > self.max_row { self.max_row = row }
        if row > self.max_column { self.max_column = column }
    }

    fn flip(&mut self) {
        let mut new_pieces : HashMap<(u64, u64), char> = HashMap::new();
        for ((row, column), element) in self.pieces.drain() {
            new_pieces.insert((row, self.max_column - column), element);
        }

        self.pieces = new_pieces;
    }

    fn rotate_90(&mut self) {
        let mut new_pieces : HashMap<(u64, u64), char> = HashMap::new();
        for ((row, column), element) in self.pieces.drain() {
            new_pieces.insert((self.max_row - column, row), element);
        }

        self.pieces = new_pieces;
    }

    fn get_puzzle_piece_side(&self, piece_side : PuzzlePieceSide) -> Vec<char> {
        let mut side : Vec<char> = Vec::new();
        // (init_row, init_column), (end_row + 1, end_column + 1)
        let iteration : ((u64, u64), (u64, u64));
        
        match piece_side {
            PuzzlePieceSide::Right => iteration = ((0, self.max_column), (self.max_row + 1, self.max_column + 1)),
            PuzzlePieceSide::Bottom => iteration = ((self.max_row, 0), (self.max_row + 1, self.max_column + 1)),
            PuzzlePieceSide::Left => iteration = ((0, 0), (self.max_row + 1, 1)),
            PuzzlePieceSide::Top => iteration = ((0, 0), (1, self.max_column + 1)),
        }

        let init : (u64, u64) = iteration.0;
        let end : (u64, u64) = iteration.1;
        for row in init.0..end.0 {
            for column in init.1..end.1 {
                let element : char = self.pieces.get(&(row, column)).unwrap().clone();
                side.push(element);
            }
        }

        return side;
    }

    fn print_piece(&self) {
        for row in 0..(self.max_row + 1) {
            for column in 0..(self.max_column + 1) {
                print!("{}", self.pieces.get(&(row, column)).unwrap());
            }

            println!();
        }
    }

    fn get_picture(&self) -> Vec<Vec<char>> {
        let mut picture_pixels : Vec<Vec<char>> = Vec::new();

        for row in 1..self.max_row {
            let mut new_row : Vec<char> = Vec::new();
            for column in 1..self.max_column {
                new_row.push(*self.pieces.get(&(row, column)).unwrap())
            }

            picture_pixels.push(new_row);
        }

        return picture_pixels;
    }
}

// ------------ Puzzle -----------------
struct Puzzle {
    pieces : HashMap<u64, PuzzlePiece>,
    solve_space : HashMap<(i32, i32), u64>,
}

impl Puzzle {
    pub fn new() -> Puzzle {
        Puzzle {
            pieces : HashMap::new(),
            solve_space : HashMap::new(),
        }
    }

    fn add_piece(&mut self, piece : PuzzlePiece) {
        let piece_number : u64 = piece.get_piece_number();
        self.pieces.insert(piece_number, piece);
    }

    fn solve(&mut self) {
        let mut rng = rand::thread_rng();
        let mut how_many_added : u32 = 0;
        // While not all pieces placed
        while self.solve_space.len() != self.pieces.len() {

            // With certain probability remove a piece
            let mut to_remove : Vec<(i32, i32)> = Vec::new();
            for (&position, _) in self.solve_space.iter() {
                let mut probability : u64 = 50;
                if how_many_added <= 0 && rng.gen_range(0..101) >= probability { to_remove.push(position); }
            }
            for remove_placement in to_remove.into_iter() {
                self.solve_space.remove(&remove_placement);
            }

            how_many_added = 0;
            // List for updated pieces
            let mut updated_pieces : Vec<PuzzlePiece> = Vec::new();

            // Iterate over pieces
            for (&piece_number, piece) in self.pieces.iter() {
                let mut clone_piece : PuzzlePiece = piece.clone();

                // Skip if piece already added to solve space
                if self.solve_space.values().any(|&number| number == piece_number) { continue }

                // If first piece to be added, simply added and go to next
                if self.solve_space.len() == 0 {
                    self.solve_space.insert((0, 0), piece_number);
                    continue;
                }

                // else must verify if valid piece
                // compute valid places
                let mut valid_placements : Vec<(i32, i32)> = Vec::new();
                for (&placement, _) in self.solve_space.iter() {
                    for row_variation in -1..2 {
                        if row_variation == 0 { continue }

                        let new_row : i32 = placement.0 + row_variation;
                        let new_column : i32 = placement.1;
                        if self.solve_space.contains_key(&(new_row, new_column)) { continue };
                        valid_placements.push((new_row, new_column));
                    }

                    for column_variation in -1..2 {
                        if column_variation == 0 { continue }

                        let new_row : i32 = placement.0;
                        let new_column : i32 = placement.1 + column_variation;
                        if self.solve_space.contains_key(&(new_row, new_column)) { continue };
                        valid_placements.push((new_row, new_column));
                    }
                }

                // iterate over valid placements
                for rotation in 0..8 {
                    // Perform Rotation of piece
                    clone_piece.rotate_90();
                    if rotation % 4 == 0 { clone_piece.flip(); }
                    
                    let mut valid_placement : Option<(i32, i32)> = None;
                    
                    for &placement in valid_placements.iter() {
                        
                        let mut is_valid_placement : bool = true;
                        // iterate over pieces already placed
                        for (&close_placement, close_to_piece_number) in self.solve_space.iter() {
                            
                            // Get side on which pieces are connected if they even are
                            let side_connection : (PuzzlePieceSide, PuzzlePieceSide);
                            if placement.0 == close_placement.0 - 1 && placement.1 == close_placement.1 {
                                side_connection = (PuzzlePieceSide::Bottom, PuzzlePieceSide::Top);
                            } else if placement.0 == close_placement.0 + 1 && placement.1 == close_placement.1 {
                                side_connection = (PuzzlePieceSide::Top, PuzzlePieceSide::Bottom);
                            } else if placement.0 == close_placement.0 && placement.1 == close_placement.1 - 1 {
                                side_connection = (PuzzlePieceSide::Right, PuzzlePieceSide::Left);
                            } else if placement.0 == close_placement.0 && placement.1 == close_placement.1 + 1 {
                                side_connection = (PuzzlePieceSide::Left, PuzzlePieceSide::Right);
                            } else {
                                continue;
                            }
                            
                            // Check if valid connection
                            let close_to_piece : &PuzzlePiece = self.pieces.get(close_to_piece_number).unwrap();
                            
                            let side1 : Vec<char> = clone_piece.get_puzzle_piece_side(side_connection.0);
                            let side2 : Vec<char> = close_to_piece.get_puzzle_piece_side(side_connection.1);
                            let valid : bool = check_valid_connection(side1.clone(), side2.clone());
                            if valid { continue }
    
                            // No valid connection
                            is_valid_placement = false;
                            break;
                        }
    
                        if is_valid_placement {
                            valid_placement = Some(placement);
                            break;
                        }
                    }

                    match valid_placement {
                        Some(placement) => {
                            how_many_added = how_many_added + 1;
                            self.solve_space.insert(placement, clone_piece.get_piece_number());
                            updated_pieces.push(clone_piece.clone());
                            break;
                        },
                        None => (),
                    }
                }
            }

            // Update pieces added to solve_space
            for updated_piece in updated_pieces.into_iter() {
                self.pieces.insert(updated_piece.get_piece_number(), updated_piece);
            }

            if self.remove_invalid_solve_space() { how_many_added = 0 }

            println!("Pieces: {}, Solved: {}", self.pieces.len(), self.solve_space.len());
        }

        println!("Solved Puzzle!");
    }

    fn remove_invalid_solve_space(&mut self) -> bool {
        let mut min : (i32, i32) = (0, 0);
        let mut max : (i32, i32) = (0, 0);
        for (&position, _) in self.solve_space.iter() {
            // Get Row
            if position.0 > max.0 { max.0 = position.0 }
            else if position.0 < min.0 { min.0 = position.0 }
            // Get Column
            if position.1 > max.1 { max.1 = position.1 }
            else if position.1 < min.1 { min.1 = position.1 }
        }

        let max_side : f32 = (self.pieces.len() as f32).sqrt();
        let mut row_variation : i32 = 0;
        let mut column_variation : i32 = 0;
        if max.0 - min.0 + 1 > max_side as i32 { row_variation = (max.0 - min.0 + 1) - max_side as i32; }
        if max.1 - min.1 + 1 > max_side as i32 { column_variation = (max.1 - min.1 + 1) - max_side as i32; }

        let mut to_remove : Vec<(i32, i32)> = Vec::new();
        for (&position, _) in self.solve_space.iter() {
            if position.0 < min.0 + row_variation { to_remove.push(position) }
            else if position.0 > max.0 - row_variation { to_remove.push(position) }
            else if position.1 < min.1 + column_variation { to_remove.push(position) }
            else if position.1 > max.1 - column_variation { to_remove.push(position) }
        }

        let removed_something : bool = to_remove.len() > 0;
        for remove_position in to_remove.iter() {
            self.solve_space.remove(remove_position);
        }

        return removed_something;
    }

    fn get_result(&self) -> u64 {
        let mut min : (i32, i32) = (0, 0);
        let mut max : (i32, i32) = (0, 0);
        for (position, _) in self.solve_space.iter() {
            // Get Row
            if position.0 > max.0 { max.0 = position.0 }
            else if position.0 < min.0 { min.0 = position.0 }
            // Get Column
            if position.1 > max.1 { max.1 = position.1 }
            else if position.1 < min.1 { min.1 = position.1 }
        }

        for row in min.0..(max.0 + 1) {
            for column in min.1..(max.1 + 1) {
                let element : Option<&u64> = self.solve_space.get(&(row, column));
                if element.is_some() { print!("{} ", self.solve_space.get(&(row, column)).unwrap()); }
                else { print!("  x  ") }
            }

            println!();
        }

        let mut result : u64 = 1;
        result = result * self.solve_space.get(&(min.0, min.1)).unwrap();
        result = result * self.solve_space.get(&(min.0, max.1)).unwrap();
        result = result * self.solve_space.get(&(max.0, min.1)).unwrap();
        result = result * self.solve_space.get(&(max.0, max.1)).unwrap();

        return result;
    }

    fn get_picture(&self) -> Picture {
        let mut picture : Picture = Picture::new();

        let mut min : (i32, i32) = (0, 0);
        let mut max : (i32, i32) = (0, 0);
        for (&placement, _) in self.solve_space.iter() {
            if placement.0 < min.0 { min.0 = placement.0 }
            if placement.1 < min.1 { min.1 = placement.1 }
            if placement.0 > max.0 { max.0 = placement.0 }
            if placement.1 > max.1 { max.1 = placement.1 }
        }

        let tmp_piece_id : &u64 = self.solve_space.get(&(min.0, min.1)).unwrap();
        let piece_size : u64 = self.pieces.get(tmp_piece_id).unwrap().get_size() - 2;

        for placement_piece_x in min.0..(max.0 + 1) {
            let placement_piece_x_norm : u64 = (placement_piece_x - min.0) as u64;
            for placement_piece_y in min.1..(max.1 + 1) {
                let placement_piece_y_norm : u64 = (placement_piece_y - min.1) as u64;

                let piece_number : u64 = *self.solve_space.get(&(placement_piece_x, placement_piece_y)).unwrap();
                let pixels : Vec<Vec<char>> = self.pieces.get(&piece_number).unwrap().get_picture();
                for (row, row_vector) in pixels.iter().enumerate() {
                    for (column, element) in row_vector.iter().enumerate() {
                        let pixel_x : u128 = placement_piece_x_norm as u128 * (piece_size + 1) as u128 + row as u128;
                        let pixel_y : u128 = placement_piece_y_norm as u128 * (piece_size + 1) as u128 + column as u128;
                        picture.add_pixel(pixel_x, pixel_y, *element);
                    }
                }
            }
        }

        return picture;
    }
}

fn check_valid_connection(side1 : Vec<char>, side2 : Vec<char>) -> bool {
    if side1.len() != side2.len() { return false }

    for (&elem1, &elem2) in side1.iter().zip(side2.iter()) {
        if elem1 != elem2 { return false }
    }

    return true;
}

// ------------ Picture -----------------
struct Picture {
    length : u128,
    pixels : HashMap<(u128, u128), char>,
}

impl Picture {
    pub fn new() -> Picture {
        Picture {
            length : 0, 
            pixels : HashMap::new(),
        }
    }

    fn add_pixel(&mut self, x : u128, y : u128, element : char) {
        self.pixels.insert((x, y), element);
        if x > self.length { self.length = x }
        if y > self.length { self.length = y }
    }

    fn print_picture(&self) {
        for x in 0..(self.length + 1) {
            for y in 0..(self.length + 1) {
                match self.pixels.get(&(x, y)) {
                    Some(element) => print!("{}", element),
                    None => panic!("Could not find ({}, {})", x, y),
                }
            }
            println!("");
        }
    }

    fn find_pattern(&mut self, mut pattern : Pattern) {
        for rotation in 0..12 {
            // Rotate pattern every iteration
            pattern.rotate_mask_90();
            if rotation % 4 == 0 { 
                if rotation % 8 == 0 {
                    pattern.flip_mask_vertical();
                } else {
                    pattern.flip_mask_horizontal();
                }
            }
            
            for x_init in 0..(self.length + 1) {
                for y_init in 0..(self.length + 1) {
                    let mut valid_pattern : bool = true;
    
                    // Check if pattern valid
                    for &mask_point in pattern.mask.iter() {
                        let check_x : u128 = x_init as u128 + mask_point.0  as u128;
                        let check_y : u128 = y_init as u128 + mask_point.1  as u128;
                        match self.pixels.get(&(check_x, check_y)) {
                            Some(&pixel) if pixel == '#' => (), 
                            Some(_) => valid_pattern = false, 
                            None => valid_pattern = false,
                        }
    
                        if !valid_pattern { break }
                    }
    
                    if !valid_pattern { continue }
                    // Replace with pattern
                    for &mask_point in pattern.mask.iter() {
                        let check_x : u128 = x_init as u128 + mask_point.0  as u128;
                        let check_y : u128 = y_init as u128 + mask_point.1  as u128;
                        self.pixels.insert((check_x, check_y), 'O');
                    }
                }
            }
        }
        
    }

    fn get_rough_water_without_monster(&self) -> u32 {
        let mut count : u32 = 0;
        for (_, &element) in self.pixels.iter() {
            if element == '#' { count = count + 1 }
        }

        return count;
    }
}

// ------------ Pattern -----------------
struct Pattern {
    width : u32,
    height : u32,
    mask : Vec<(u32, u32)>,
}

impl Pattern {
    pub fn new() -> Pattern {
        Pattern {
            width : 0,
            height : 0,
            mask : Vec::new(),
        }
    }

    fn add_mask_point(&mut self, x : u32, y : u32) {
        if x > self.height { self.height = x }
        if y > self.width { self.width = y }
        self.mask.push((x, y));
    }

    fn flip_mask_vertical(&mut self) {
        let mut new_mask : Vec<(u32, u32)> = Vec::new();
        for &pos in self.mask.iter() {
            new_mask.push((pos.0, self.width - pos.1));
        }

        self.mask = new_mask;
    }

    fn flip_mask_horizontal(&mut self) {
        let mut new_mask : Vec<(u32, u32)> = Vec::new();
        for &pos in self.mask.iter() { new_mask.push((self.height - pos.0, pos.1)) }

        self.mask = new_mask;
    }

    fn rotate_mask_90(&mut self) {
        let mut non_normalized_new_mask : Vec<(i32, i32)> = Vec::new();
        let mut new_mask : Vec<(u32, u32)> = Vec::new();

        // Simply rotate
        for &pos in self.mask.iter() { non_normalized_new_mask.push((self.height as i32 - pos.1 as i32, pos.0 as i32)) }

        // Correct Rotation to be on (0, 0)
        let mut min : (i32, i32) = *non_normalized_new_mask.first().unwrap();
        for &pos in non_normalized_new_mask.iter() {
            if pos.0 < min.0 { min.0 = pos.0 }
            if pos.1 < min.1 { min.1 = pos.1 }
        }

        for &pos in non_normalized_new_mask.iter() { new_mask.push(((pos.0 - min.0) as u32, (pos.1 - min.1) as u32)) }
        self.mask = new_mask;

        let tmp : u32 = self.height;
        self.height = self.width;
        self.width = tmp;
    }
}

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut puzzle : Puzzle = Puzzle::new();
       
    // Initialize for first piece
    let mut phase : ProcessingPhase = ProcessingPhase::GetPuzzle;
    let number : u64 = data.remove(0).replace("Tile ", "").replace(":", "").parse().unwrap();
    let mut current_piece : PuzzlePiece = PuzzlePiece::new(number);

    let mut row_count : u64 = 0;
    for line in data.iter() {
        match phase {
            ProcessingPhase::GetPuzzleNumber => {
                // Add last piece
                puzzle.add_piece(current_piece);
                
                // Prepare for next piece
                let number : u64 = line.replace("Tile ", "").replace(":", "").parse().unwrap();
                current_piece = PuzzlePiece::new(number);
                phase = ProcessingPhase::GetPuzzle;
                row_count = 0;
            },
            ProcessingPhase::GetPuzzle => {
                // Stopping case
                if line == "" {
                    phase = ProcessingPhase::GetPuzzleNumber;
                    continue;
                }

                for (column_count, element) in line.chars().enumerate() {
                    current_piece.add_element(row_count, column_count as u64, element);
                }
        
                row_count = row_count + 1;
            }
        }
    }

    // Adding last piece
    puzzle.add_piece(current_piece);

    // Solve and solution (Part 1)
    puzzle.solve();
    println!("------------------------------");
    println!("Result {}", puzzle.get_result());
    
    // Solve and solution (Part 2)
    let mut picture : Picture = puzzle.get_picture();
    println!("------------------------------");
    picture.print_picture();

    // Read pattern
    let filename = "src/monster.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut monster_pattern : Pattern = Pattern::new();
    for (x, line) in data.iter().enumerate() {
        for (y, element) in line.chars().enumerate() {
            if element != '#' { continue }
            monster_pattern.add_mask_point(x as u32, y as u32);
        }
    }

    println!("------------------------------");
    picture.find_pattern(monster_pattern);
    picture.print_picture();
    println!("Result {}", picture.get_rough_water_without_monster());
}
