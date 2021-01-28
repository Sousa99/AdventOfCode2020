use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

// TODO:
// Maybe a bit of refactoring, altough not bad, it could be more straighforward

const DEBUG : bool = false;

// ------------------ Processing Phase ------------------
enum ProcessingPhase {
    PlayerIdentifier,
    PlayerCards
}

// ------------------ Card ------------------
#[derive(Copy, Clone)]
struct Card {
    card_number : u32,
}

impl Card {
    pub fn new(card_number : u32) -> Card {
        Card {
            card_number : card_number,
        }
    }

    fn get_value(&self) -> u32 { self.card_number }
}

// ------------------ Player ------------------
struct Player {
    identifier : String,
    deck : VecDeque<Card>,
}

impl Player {
    pub fn new(identifier : String) -> Player {
        Player {
            identifier : identifier,
            deck : VecDeque::new(),
        }
    }

    fn add_card(&mut self, card : Card) { self.deck.push_back(card) }
    fn add_cards(&mut self, cards : Vec<Card>) {
        for card in cards.into_iter() { self.deck.push_back(card) }
    }
    
    fn get_identifier(&self) -> String { self.identifier.clone() }
    fn get_card(&mut self) -> Option<Card> { self.deck.pop_front() }
    fn get_copy_cards(&self) -> Vec<Card> { self.deck.clone().into_iter().collect() }
    fn get_number_cards(&self) -> usize { self.deck.len() }
    fn player_lost(&self) -> bool { self.deck.len() == 0 }
    fn print_player(&self) {
        let cards_list : Vec<String> = self.deck.iter().map(|card| card.get_value().to_string()).collect();
        println!("Player {}'s deck: {}", self.identifier, cards_list.join(", "))
    }

    fn get_result(&self) -> u32 {
        let mut result : u32 = 0;
        let number_cards : u32 = self.deck.len() as u32;

        for (index, card) in self.deck.iter().enumerate() {
            result = result + card.get_value() * (number_cards - index as u32);
        }

        return result;
    }
}

// ------------------ Game ------------------
struct Game {
    states : Vec<Vec<Vec<Card>>>,
    players : Vec<Player>,
    round : u32,
    identifier : u32,
    count_games : u32,
}

impl Game {
    pub fn new(identifier : u32) -> Game {
        Game {
            states : Vec::new(),
            players : Vec::new(),
            round : 0,
            identifier : identifier,
            count_games : identifier + 1,
        }
    }

    fn add_player(&mut self, player : Player) { self.players.push(player) }
    fn get_who_won(&self) -> usize {
        // Figure out if only one won
        let mut how_many_still_have_cards : u32 = 0;
        for player in self.players.iter() {
            if player.get_number_cards() != 0 {
                how_many_still_have_cards = how_many_still_have_cards + 1;
            }
        }

        for (index, player) in self.players.iter().enumerate() {
            if how_many_still_have_cards == 1 && player.get_number_cards() != 0 { return index }
            if how_many_still_have_cards > 1 && player.get_identifier() == "1" { return index }   
        }

        panic!("This should not be reached!");
    }

    fn get_result(&self) {
        for player in self.players.iter() {
            println!("Player {}'s result: {}", player.get_identifier(), player.get_result());
        }
    }

    fn state_has_happened_before(&mut self) -> bool {
        // Get current state
        let mut curr_state : Vec<Vec<Card>> = Vec::new();
        for player in self.players.iter() { curr_state.push(player.get_copy_cards()) }

        // Check if state happened before
        let mut has_happened : bool = false;
        for state in self.states.iter() {
            // Compare states
            let mut differ : bool = false;
            for (curr_player_state, player_state) in curr_state.iter().zip(state.iter()) {
                for (curr_card_state, card_state) in curr_player_state.iter().zip(player_state.iter()) {
                    if curr_card_state.get_value() != card_state.get_value() {
                        differ = true;
                        break;
                    }
                }

                if differ { break }
            }

            if !differ { has_happened = true }
        }
        
        self.states.push(curr_state);
        return has_happened;
    }

    fn run_sub_game(&mut self, cards : Vec<Option<Card>>) -> usize {
        if DEBUG {
            println!("Playing a sub-game to determine the winner...");
            println!();
        }

        let mut sub_game : Game = Game::new(self.count_games);
        self.count_games = self.count_games + 1;

        for (player, card_option) in self.players.iter().zip(cards.iter()) {
            match card_option {
                Some(card) => {
                    let new_player_name : String = player.get_identifier();
                    let mut new_player : Player = Player::new(new_player_name);
                    let card_value : usize = card.get_value() as usize;
                    let cards : Vec<Card> = player.get_copy_cards()[ ..card_value].to_vec();
        
                    new_player.add_cards(cards);
                    sub_game.add_player(new_player);
                },

                None => (),
            }
        }

        sub_game.run();
        return sub_game.get_who_won();

    }

    fn run(&mut self) {
        if DEBUG {
            println!("=== Game {} ===", self.identifier);
            println!();
        }

        let mut all_except_one_lost : bool = false;
        while !all_except_one_lost {
            
            // Update round
            self.round = self.round + 1;
            if DEBUG { println!("-- Round {} (Game {})--", self.round, self.identifier) }

            // Check if the very same has happen before
            if self.state_has_happened_before() {
                if DEBUG { println!("State has happened before, player 1 wins automatically!") }
                break;
            }

            // Get cards of each player
            let mut cards : Vec<Option<Card>> = Vec::new();
            for player in self.players.iter_mut() {
                if DEBUG { player.print_player() }
                cards.push(player.get_card())
            }

            // Print out round cards played
            if DEBUG {
                for (player, card) in self.players.iter().zip(cards.iter()) {
                    match card {
                        Some(card) => println!("Player {} plays: {}", player.get_identifier(), card.get_value()),
                        None => (),
                    }
                }
            }

            // Figure out if it should run a sub-game
            let mut trigger_sub_game : bool = true;
            for (player, option_card) in self.players.iter().zip(cards.iter()) {
                match option_card {
                    Some(card) => {
                        let number_cards_left : u32 = player.get_number_cards() as u32;
                        if number_cards_left < card.get_value() { trigger_sub_game = false }
                    },

                    None => (),
                }
            }

            let who_won : usize;
            if trigger_sub_game {
                // Run sub-game if we are supposed to
                who_won = self.run_sub_game(cards.clone());

            } else {
                // Run game as usual

                // Find who won
                let mut track_scores : (usize, u32) = (0, 0);
                for (index, option_card) in cards.iter().enumerate() {
                    match option_card {
                        Some(card) if card.get_value() > track_scores.1 => track_scores = (index, card.get_value()),
                        Some(_) => (),
                        None => (),
                    }
                }

                who_won = track_scores.0;
            }

            // Give cards to who won
            let mut cards : Vec<Card> = cards.into_iter().filter_map(|card| card).collect();
            cards.rotate_left(who_won);
            self.players.get_mut(who_won).unwrap().add_cards(cards);
            if DEBUG {
                println!("Player {} wins round {} of game {}", 
                    self.players.get(who_won).unwrap().get_identifier(),
                    self.round,
                    self.identifier
                )
            }

            // Figure out how many have lost
            let mut count_active_players : u32 = 0;
            for player in self.players.iter() {
                if !player.player_lost() { count_active_players = count_active_players + 1 }
            }

            all_except_one_lost = count_active_players == 1;
            if DEBUG { println!() }
        }

        if DEBUG {
            println!("== Post-game results ==");
            for player in self.players.iter() { player.print_player() }
        }
    }
}

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut data : Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();
    
    let mut game : Game = Game::new(1);
    // Get first player
    let player_name : String = data.remove(0).replace("Player ", "").replace(":", "");
    let mut player : Player = Player::new(player_name);
    let mut current_phase : ProcessingPhase = ProcessingPhase::PlayerCards;

    for line in data.iter() {
        match current_phase {
            ProcessingPhase::PlayerIdentifier => {
                // Add player
                game.add_player(player);
                let new_player_name : String = line.replace("Player ", "").replace(":", "");
                player = Player::new(new_player_name);
                // Update phase
                current_phase = ProcessingPhase::PlayerCards;
            },

            ProcessingPhase::PlayerCards => {
                // Check if cards ended
                if line == "" {
                    current_phase = ProcessingPhase::PlayerIdentifier;
                    continue;
                }

                // If not
                let card_number : u32 = line.parse().unwrap();
                let card : Card = Card::new(card_number);
                player.add_card(card);
            }
        }
    }

    // Add last player
    game.add_player(player);

    // Solve game
    game.run();
    game.get_result();
}
