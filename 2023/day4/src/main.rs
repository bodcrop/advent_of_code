use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("src/input.txt").expect("Could not find file");
    let mut total_score_v1 = 0;
    let mut total_number_of_cards = 0;
    let mut row_count: u32 = 0;

    //initialise start cards
    let mut track_score_q2: HashMap<u32, u32> = HashMap::new();
    for number_card in 0..content.split("\n").collect::<Vec<&str>>().len() - 1 {
        println!("{number_card}");
        track_score_q2.insert(number_card.try_into().unwrap(), 1);
    }

    for row in content.split("\n") {
        let game_parts = row.split(":").collect::<Vec<&str>>();
        if game_parts.len() == 1 {
            break;
        }
        let game_play: Vec<&str> = game_parts[1].split("|").collect();

        let mut winning_numbers = HashMap::new();

        let regex_digits: Regex = Regex::new(r"\d+").unwrap();

        for (start_number, end_number) in regex_digits.find_iter(game_play[0]) {
            winning_numbers.insert(game_play[0].get(start_number..end_number).unwrap(), 0);
        }
        let mut number_winning_numbers: u32 = 0;
        let current_cards = *track_score_q2.get(&row_count).unwrap();

        for played_number in game_play[1].split(" ") {
            if winning_numbers.contains_key(played_number) {
                number_winning_numbers += 1;
                let card_number_won = row_count + number_winning_numbers;
                let card_won = track_score_q2.get_mut(&card_number_won).unwrap();
                *card_won += current_cards;
                println!("{card_won} {card_number_won}");
            }
        }
        println!(
            "For card row {row_count} you won {number_winning_numbers} and have now {} cards",
            track_score_q2.entry(row_count).or_insert(1)
        );
        if number_winning_numbers != 0 {
            total_score_v1 += i32::pow(2, number_winning_numbers - 1)
        }

        row_count += 1;
    }

    for n_card in track_score_q2.values() {
        total_number_of_cards += n_card;
    }
    println!("Q1: You have won {}", total_score_v1);
    println!("Q2: You have won {} cards.", total_number_of_cards);
}
