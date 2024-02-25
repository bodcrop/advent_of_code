use std::collections::HashMap;
use std::fs;
fn main() {
    println!("Hello, world!");
    let content: String =
        fs::read_to_string("src/input.txt").expect("Could not read the input file");
    let rows: Vec<&str> = content.split("\n").collect();
    question1(&rows);
}

fn question1(rows: &Vec<&str>) {
    let mut number_of_valid_games: i32 = 0;
    for row in rows {
        let mut row_valid = 1;
        let row_info = row.split(":").collect::<Vec<&str>>();
        if row_info.len() != 2 {
            break;
        }
        let game_id_info: Vec<&str> = row_info[0].split(" ").collect();
        let game = row_info[1];

        for session in game.split(";") {
            let mut session_colours = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);
            for colour_amounts in session.split(",") {
                let colour_amount = colour_amounts.trim().split(" ").collect::<Vec<&str>>();
                let amount = colour_amount[0].parse::<i32>().unwrap();
                let colour = colour_amount[1].trim();
                let session_colour_amount: &mut i32 = session_colours.get_mut(colour).unwrap();
                *session_colour_amount += amount;
                // println!("For colour {colour}: {amount}");
            }

            if session_colours["red"] <= 12
                && session_colours["green"] <= 13
                && session_colours["blue"] <= 14
            {
                row_valid *= row_valid
            } else {
                row_valid = 0;
            }
        }

        if game_id_info.len() == 2 {
            let game_id = game_id_info[1];
            println!("{game_id}: {row_valid}");
            number_of_valid_games += game_id.parse::<i32>().unwrap() * row_valid;
        }
    }

    println!("{number_of_valid_games} are valid games given the constraints");
}
