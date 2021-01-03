use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

struct Field {
    field_code: String,
    field_name: String,
    value: String,
    optional: bool,
    callback: fn(String) -> bool,
}

impl Field {
    fn validate(&self) -> bool {
        return (self.callback)(self.value.clone());
    }
}

struct Param {
    code: String,
    value: String,
}

fn build_field(field_code : &str, field_name: &str, optional : bool, callback : fn(String) -> bool) -> Field {
    let new_field = Field {
        field_code: field_code.to_string(),
        field_name: field_name.to_string(),
        value: "None".to_string(),
        optional: optional,
        callback: callback,
    };

    return new_field;
}

fn build_field_from_template(field_mockup : &Field, value : String) -> Field {
    let new_field = Field {
        field_code: field_mockup.field_code.clone(),
        field_name: field_mockup.field_name.clone(),
        value: value,
        optional: field_mockup.optional,
        callback: field_mockup.callback,
    };

    return new_field;
}

fn build_param(code : String, value : String) -> Param {
    let new_param = Param {
        code: code,
        value: value,
    };

    return new_param;
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut fields : Vec<Field> = Vec::new();
    fields.push(build_field("byr", "Birth Year", false, validate_byr));
    fields.push(build_field("iyr", "Issue Year", false, validate_iyr));
    fields.push(build_field("eyr", "Expiration Year", false, validate_eyr));
    fields.push(build_field("hgt", "Height", false, validate_hgt));
    fields.push(build_field("hcl", "Hair Color", false, validate_hcl));
    fields.push(build_field("ecl", "Eye Color", false, validate_ecl));
    fields.push(build_field("pid", "Passport ID", false, validate_pid));
    fields.push(build_field("cid", "Country ID", true, validate_cid));

    let mut passports : Vec<Vec<Field>> = Vec::new();
    let mut current_passport : Vec<Param> = Vec::new();
    for line in data.iter() {
        let char_count = line.chars().count();
        if char_count == 0 {
            add_check_passport(&mut passports, &mut fields, &mut current_passport);
            current_passport = Vec::new();
            continue;
        }

        let information = line.split(" ");
        for parameter in information {
            let mut parameter_split = parameter.split(":");
            let parameter_code = parameter_split.nth(0).unwrap_or("No element there (index 0)").to_string();
            let parameter_value = parameter_split.nth(0).unwrap_or("No element there (index 0)").to_string();

            current_passport.push(build_param(parameter_code, parameter_value));
        }
    }

    add_check_passport(&mut passports, &mut fields, &mut current_passport);

    println!("Number of valid Passports: {}", passports.len());
}

fn add_check_passport(passports : &mut Vec<Vec<Field>>, fields : &mut Vec<Field>, current_passport : &mut Vec<Param>) {
    let mut passport : Vec<Field> = Vec::new();

    for field in fields.iter() {
        let information = current_passport.iter().find(|info| info.code == field.field_code);
        match information {
            Some(information_some) => {
                let new_field = build_field_from_template(field, information_some.value.clone());
                if !new_field.validate() { return; };

                passport.push(new_field);
            },
            None => {
                if !field.optional {
                    //println!("Non-optional information not given - {}", field.field_name);
                    return;
                }
            }
        }
    }

    passports.push(passport);
}


// Support Functions
fn parse_i32(value : String) -> Result<i32, ParseIntError> {
    return value.parse();
}

fn between_values(value : i32, lower : i32, upper : i32) -> bool {
    return value >= lower && value <= upper;
}


// Support Validation Functions
fn validate_byr(value : String) -> bool {
    let value_parsed : i32 = match parse_i32(value) {
        Ok(number) => number,
        Err(_e) => return false,
    };

    return between_values(value_parsed, 1920, 2002);
}

fn validate_iyr(value : String) -> bool {
    let value_parsed : i32 = match parse_i32(value)  {
        Ok(number) => number,
        Err(_e) => return false,
    };

    return between_values(value_parsed, 2010, 2020);
}

fn validate_eyr(value : String) -> bool {
    let value_parsed : i32 = match parse_i32(value) {
        Ok(number) => number,
        Err(_e) => return false,
    };
    
    return between_values(value_parsed, 2020, 2030);
}

fn validate_hgt(value : String) -> bool {
    let len_value : usize = value.chars().count();
    let scale : String = value[len_value - 2 ..].to_string();
    let value_parsed : i32 = match parse_i32(value[.. len_value - 2].to_string()) {
        Ok(number) => number,
        Err(_e) => return false,
    };

    if scale == "cm" {return between_values(value_parsed, 150, 193)}
    else if scale == "in" {return between_values(value_parsed, 59, 76)}
    else { return false; }
}

fn validate_hcl(value : String) -> bool {
    let initial_characther : String = "#".to_string();
    let number_characthers : u32 = 6;

    let mut iter_chars = value.chars();

    if iter_chars.nth(0).unwrap().to_string() != initial_characther { return false; }
    if iter_chars.clone().count() as u32 != number_characthers { return false; }
    for characther in iter_chars {
        if !((characther >= 'a' && characther <= 'f') || (characther >= '0' && characther <= '9')) {
            return false;
        }
    }

    return true;
}

fn validate_ecl(value : String) -> bool {
    let available_colors : Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    return available_colors.iter().any(|&x| x == value);
}

fn validate_pid(value : String) -> bool {
    let number_characthers : u32 = 9;

    let iter_chars = value.chars();
    if iter_chars.clone().count() as u32 != number_characthers { return false; }
    for characther in iter_chars {
        if !(characther >= '0' && characther <= '9') {
            return false;
        }
    }

    return true;
}

fn validate_cid(_value : String) -> bool {
    return true;
}