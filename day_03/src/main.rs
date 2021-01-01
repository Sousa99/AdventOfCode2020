use std::fs::File;
use std::io::{BufRead, BufReader};

struct Slope {
    count: u32,
    step: u32,
    down_step: u32,
    current_index: u32,
}

fn build_slope(step : u32, down_step : u32) -> Slope {
    let slope = Slope {
        count: 0,
        step: step,
        down_step: down_step,
        current_index: 0
    };

    return slope;
}

fn print_slope(slope: &Slope) {
    println!("({}, {}): {} trees in the way", slope.step, slope.down_step, slope.count);
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    // Initialize Vector and Slopes
    let mut slopes : Vec<Slope> = Vec::new();
    slopes.push(build_slope(1, 1));
    slopes.push(build_slope(3, 1));
    slopes.push(build_slope(5, 1));
    slopes.push(build_slope(7, 1));
    slopes.push(build_slope(1, 2));

    for (index_line, line) in data.iter().enumerate() {
        for slope in slopes.iter_mut() {
            if index_line as u32 % slope.down_step != 0 { continue; }

            let line_size : u32 = line.chars().count() as u32;
            let index_to_search : u32 = slope.current_index % line_size;
            let characther = line.chars().nth(index_to_search as usize).unwrap();

            if characther == '#' { slope.count = slope.count + 1; }
            slope.current_index = slope.current_index + slope.step;
        }
    }

    let mut multiplication : u32 = 1;
    for slope in slopes.iter() {
        print_slope(slope);
        multiplication = multiplication * slope.count;
    }

    println!("Number of Trees Obstructing (multiplication): {}", multiplication);
}