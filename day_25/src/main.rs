use std::fs::File;
use std::io::{BufRead, BufReader};

// ------------------ Device ------------------
struct Device {
    loop_size : u64,
    subject_number : u64,
    current_value : u64,
    remainder_value : u64,
}

impl Device {
    pub fn new(subject_number : u64, remainder_used : u64) -> Device {
        Device {
            loop_size : 1,
            subject_number : subject_number,
            current_value : subject_number,
            remainder_value : remainder_used,
        }
    }

    fn get_loop_size(&self) -> u64 { self.loop_size }
    fn set_loop_size(&mut self, loop_size : u64) { self.loop_size = loop_size }

    fn make_loop(&mut self) -> u64 {
        self.loop_size = self.loop_size + 1;
        
        // value <- value * subject_number
        self.current_value = self.current_value * self.subject_number;
        // value <- value % remainder_value
        self.current_value = self.current_value % self.remainder_value;

        return self.current_value;
    }

    fn encrypt_value(&mut self) -> u64 {
        for _ in 0..(self.loop_size - 1) { self.make_loop(); }
        return self.current_value;
    }

}

// ------------------ Mechanism ------------------
struct Mechanism {
    card : Device,
    card_public_key : u64,
    reader : Device,
    reader_public_key : u64,
}

impl Mechanism {
    pub fn new(card_public_key : u64, reader_public_key : u64) -> Mechanism {
        Mechanism {
            card : Device::new(7, 20201227),
            card_public_key : card_public_key,
            reader : Device::new(7, 20201227),
            reader_public_key : reader_public_key,
        }
    }

    fn find_loop_cycles(&mut self) {
        // Find card loop cycle
        let mut tmp_current_value : u64 = 7;
        while tmp_current_value != self.card_public_key { tmp_current_value = self.card.make_loop() }
        let loop_size : u64 = self.card.get_loop_size();
        println!("Card loop cycle: '{}'", loop_size);
        
        // Find reader loop cycle
        let mut tmp_current_value : u64 = 7;
        while tmp_current_value != self.reader_public_key { tmp_current_value = self.reader.make_loop() }
        let loop_size : u64 = self.reader.get_loop_size();
        println!("Reader loop cycle: '{}'", loop_size);
    }

    fn find_encryption_key(&mut self) -> u64 {

        let card_loop_size : u64 = self.card.get_loop_size();
        let reader_loop_size : u64 = self.reader.get_loop_size();

        // Card Mock
        let mut card_mock : Device = Device::new(self.reader_public_key, 20201227);
        card_mock.set_loop_size(card_loop_size);
        let card_encryption_key : u64 = card_mock.encrypt_value();
        // Reader Mock
        let mut reader_mock : Device = Device::new(self.card_public_key, 20201227);
        reader_mock.set_loop_size(reader_loop_size);
        let reader_encryption_key : u64 = reader_mock.encrypt_value();

        assert_eq!(card_encryption_key, reader_encryption_key);
        return card_encryption_key;
    }
}


fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    let card_public_key : u64 = data.remove(0).parse().unwrap();
    let reader_public_key : u64 = data.remove(0).parse().unwrap();

    let mut mechanism : Mechanism = Mechanism::new(card_public_key, reader_public_key);
    mechanism.find_loop_cycles();
    let encryption_key : u64 = mechanism.find_encryption_key();
    println!("Encryption Key: '{}'", encryption_key);
}
