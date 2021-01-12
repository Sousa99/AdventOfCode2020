use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

type MemoryPointer = u64;
type Value = u64;

// -------------------- Mask --------------------
struct MaskValue {
    bit : Value,
    value : Option<Value>,
}

impl MaskValue {
    pub fn new(bit : Value, value : Option<Value>) -> MaskValue {
        MaskValue {
            bit : bit,
            value : value,
        }
    }
}

// -------------------- Memory Block --------------------
struct MemoryBlock {
    pointer : MemoryPointer,
    value : Value,
}

impl MemoryBlock {
    pub fn new(pointer : MemoryPointer) -> MemoryBlock {
        MemoryBlock {
            pointer : pointer,
            value : 0,
        }
    }

    fn get_pointer(&self) -> MemoryPointer { self.pointer }
    fn get_value(&self) -> Value { self.value }
    fn set_value(&mut self, value : Value) { self.value = value; }
}

// -------------------- System --------------------
enum SystemVersion {
    Version1,
    Version2,
}

struct System {
    version : SystemVersion,
    mask : Vec<MaskValue>,
    memory : Vec<MemoryBlock>
}

impl System {
    pub fn new(version : SystemVersion) -> System {
        System {
            version : version,
            mask : Vec::new(),
            memory : Vec::new(),
        }
    }

    fn set_mask(&mut self, mask : String) {
        self.mask = Vec::new();

        for (index, characther) in mask.chars().rev().enumerate() {
            let value : Option<Value> = match characther {
                'X' => None,
                '0' => Some(0),
                '1' => Some(1),
                _ => panic!("Invalid characther for a mask."),
            };

            let bit : Value = (2 as Value).pow(index as u32);
            self.mask.push(MaskValue::new(bit, value));
        }
    }

    fn set_memory(&mut self, original_pointer : MemoryPointer, value : Value) {
        let mut pointers : Vec<MemoryPointer> = vec!(original_pointer);
        let mut new_value : Value = value;

        match self.version {
            SystemVersion::Version1 => { new_value = self.convert_value(value); },
            SystemVersion::Version2 => { pointers = self.convert_pointer(original_pointer); }
        }

        for pointer in pointers.into_iter() {
            let mut found : bool = false;
            for block in self.memory.iter_mut() {
                if block.get_pointer() == pointer {
                    block.set_value(new_value);
                    found = true;
                    break;
                }
            }

            if found { continue }
            let mut new_memory_block : MemoryBlock = MemoryBlock::new(pointer);
            new_memory_block.set_value(new_value);
            self.memory.push(new_memory_block);
        }
    }

    fn convert_value(&self, value : Value) -> Value {
        let mut converted_value : Value = value;
        let mut sum_0 : Value = 0; 
        let mut sum_1 : Value = 0;
        
        for mask in self.mask.iter() {
            match mask.value {
                Some(0) => sum_0 = sum_0 + mask.bit,
                Some(1) => sum_1 = sum_1 + mask.bit,
                Some(_) => panic!("Invalid characther for a mask."),
                None => (),
            }
        }

        // Set 0's
        converted_value = converted_value & ((2 as Value).pow(36) - 1 - sum_0);
        // Set 1's
        converted_value = converted_value | sum_1;

        return converted_value;
    }

    fn convert_pointer(&self, original_pointer : MemoryPointer) -> Vec<MemoryPointer> {
        let mut pointers : Vec<MemoryPointer> = Vec::new();
        let mut sum_1 : Value = 0;
        
        for mask in self.mask.iter() {
            match mask.value {
                Some(1) => sum_1 = sum_1 + mask.bit,
                _ => (),
            }
        }

        // Set 1's
        pointers.push(original_pointer | sum_1);

        for mask in self.mask.iter() {
            match mask.value {
                None => {
                    let mut to_add_pointers : Vec<MemoryPointer> = Vec::new();
                    for pointer in pointers.iter() {
                        to_add_pointers.push(pointer ^ mask.bit);
                    }

                    pointers.append(&mut to_add_pointers);
                },
                _ => ()
            }
        }

        
        return pointers;
    }

    fn get_sum_in_memory(&self) -> Value {
        let mut sum : Value = 0;
        for mem_block in self.memory.iter() {
            sum = sum + mem_block.get_value() as Value;
        }

        return sum;
    }
}

fn main() {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let regex_mask = Regex::new(r"^mask = ([X|1|0]+)$").unwrap();
    let regex_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut system1 : System = System::new(SystemVersion::Version1);
    let mut system2 : System = System::new(SystemVersion::Version2);

    for line in data.iter() {
        if regex_mask.is_match(line) {
            let cap : regex::Captures = regex_mask.captures(line).unwrap();

            let mask : String = (&cap[1]).to_string();
            system1.set_mask(mask.clone());
            system2.set_mask(mask.clone());

        } else if regex_mem.is_match(line) {
            let cap : regex::Captures = regex_mem.captures(line).unwrap();

            let memory_pointer : MemoryPointer = match (&cap[1]).parse() {
                Ok(value) => value,
                Err(_) => panic!("Pointer value could not be converted.")
            };
            let value : Value = match (&cap[2]).parse() {
                Ok(value) => value,
                Err(_) => panic!("Memory value could not be converted.")
            };

            system1.set_memory(memory_pointer, value);
            system2.set_memory(memory_pointer, value);

        } else {
            panic!("Line '{}' not recognized by the program.", line);
        }
    }

    let resulting_value : Value = system1.get_sum_in_memory();
    println!("The resulting value is '{}' (part 1)", resulting_value);
    let resulting_value : Value = system2.get_sum_in_memory();
    println!("The resulting value is '{}' (part 2)", resulting_value);
}
