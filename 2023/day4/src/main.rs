use regex::Regex;
use std::collections::HashMap;
use std::fs;
fn main() {
    let content = fs::read_to_string("src/input.txt").expect("Could not find file");
    let mut total_Score = 0;
    let mut row_count: i32 = 0;
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
        for played_number in game_play[1].split(" ") {
            if winning_numbers.contains_key(played_number) {
                number_winning_numbers += 1;
                println!("For row {row_count}: {number_winning_numbers} contains {played_number}")
            }
        }
        if number_winning_numbers != 0 {
            total_Score += i32::pow(2, number_winning_numbers - 1)
        }
        row_count += 1;
    }
    println!("You have won {}", total_Score);
}
