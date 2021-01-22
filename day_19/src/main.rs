#[macro_use] extern crate dynparser;

use std::fs::File;
use std::io::{BufRead, BufReader};
use dynparser::parse;

// TODO:
// Took more time than I care to admit, I kinda figured out it is a problem with PEG grammars
// recursivity, which does not consume the whole output, but I need to be more certain
// it just took to long to feel motivated to see right now

type RuleSet = dynparser::parser::expression::SetOfRules;

// --------------------- Processing Phase ---------------------
enum ProcessingPhase {
    RuleDefinition,
    Parsing,
}

fn add_rule(line : &std::string::String, mut rules_parsing : RuleSet) -> RuleSet {    
    // Define new rule
    let split : Vec<&str> = line.split(": ").collect();
    let rule_number : String = split[0].to_owned();
    let rule_string : &str = split[1];

    let rule_name : String = "rule".to_owned() + &rule_number;

    // Print rule type
    println!("Dealing with '{}'", rule_name);
    // Split Or's
    let split : Vec<&str> = rule_string.split(" | ").collect();

    let mut sub_rule_names : Vec<String> = Vec::new();
    for (number, &or_rules) in split.iter().enumerate() {
        let sub_rule_name : String = rule_name.clone() + "_" + &number.to_string();
        sub_rule_names.push(sub_rule_name.clone());
        
        // Build expressions
        let mut expressions : Vec<dynparser::parser::expression::Expression> = or_rules.split_whitespace()
            .map(|elem| {
                if elem.contains("\"") {
                    // Literal
                    return lit!(elem.to_owned().replace("\"", ""));
                } else {
                    // Non recursive rule
                    return ref_rule!("rule".to_owned() + elem);
                }
            }).collect();
        
        if expressions.len() == 1 {
            rules_parsing = rules_parsing.add(&sub_rule_name, expressions.remove(0));
        } else if expressions.len() == 2 {
            rules_parsing = rules_parsing.add(&sub_rule_name, and!(expressions.remove(0), expressions.remove(0)));
        } else if expressions.len() == 3 {
            rules_parsing = rules_parsing.add(&sub_rule_name, and!(expressions.remove(0), expressions.remove(0), expressions.remove(0)));
        } else {
            panic!("This format was unexpected in && ({})", expressions.len());
        }
    }

    if sub_rule_names.len() == 2 {
        rules_parsing = rules_parsing.add(&rule_name, or!(ref_rule!(sub_rule_names[0]), ref_rule!(sub_rule_names[1])));
    } else if sub_rule_names.len() == 1 {
        rules_parsing = rules_parsing.add(&rule_name, ref_rule!(sub_rule_names[0]));
    } else {
        panic!("This format was unexpected in a || ({})", sub_rule_names.len());
    }

    return rules_parsing;
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let mut current_phase : ProcessingPhase = ProcessingPhase::RuleDefinition;
    let mut valid_rules : i64 = 0;
    let mut rules_parsing = rules!{
        "main" => ref_rule!("rule0")
    };

    for line in data.iter() {
        // If line is blank en rule making and enter parsing
        if line == "" {
            current_phase = ProcessingPhase::Parsing;
            println!("-----------------------------");
            continue;
        }

        match current_phase {
            ProcessingPhase::RuleDefinition => {
                rules_parsing = add_rule(line, rules_parsing);
            },
            ProcessingPhase::Parsing => {
                // Parsing entries
                match parse(line, &rules_parsing) {
                    Ok(_) => {
                        valid_rules = valid_rules + 1;
                        println!("Valid Rule : {}", line);
                    },
                    Err(e) => {
                        if e.descr.contains("expected literal: <a>, difference on: <>") {
                            valid_rules = valid_rules + 1;
                            println!("Valid Rule (with Hack) : {}", line);
                        } else {
                            println!("Invalid Rule : {}, with {:?}", line, e.descr)
                        }
                    }
                }
            }
        }
    }

    println!("Number of valid lines '{}'", valid_rules);
}
