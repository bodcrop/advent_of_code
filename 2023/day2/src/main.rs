    use std::cmp;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Hello, world!");
    let content: String =
        fs::read_to_string("src/input.txt").expect("Could not read the input file");
    let rows: Vec<&str> = content.split('\n').collect();
    q1(&rows);
    q2(&rows);
}

fn q1(rows: &[&str]) {
    let mut number_of_valid_games: i32 = 0;
    for row in rows {
        if let Some((game_id, valid_game)) = valid_game(row) {
            number_of_valid_games += game_id.parse::<i32>().unwrap() * valid_game;
        }
    }
    println!("{number_of_valid_games} are valid games given the constraints");
}

fn q2(rows: &Vec<&str>) {
    let mut sum_power = 0;
    for row in rows {
        let mut mimimum_needed_colours = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);
        if let Some((__game_id, game_sessions)) = get_game_info(row) {
            for session in game_sessions.split(';') {
                let session_colours = process_game_session(session);
                for colour in ["red", "blue", "green"] {
                    *mimimum_needed_colours.get_mut(colour).unwrap() =
                        cmp::max(mimimum_needed_colours[colour], session_colours[colour])
                }
            }
            sum_power += mimimum_needed_colours["red"]
                * mimimum_needed_colours["green"]
                * mimimum_needed_colours["blue"];
        }
    }
    println!("Total sum power is {sum_power}");
}

fn get_game_info(row: &str) -> Option<(&str, &str)> {
    let row_info = row.split(':').collect::<Vec<&str>>();
    if row_info.len() != 2 {
        return None;
    }

    let game_id_info: Vec<&str> = row_info[0].split(' ').collect();
    if game_id_info.len() != 2 {
        return None;
    }

    let game_id = game_id_info[1];
    let game_sessions = row_info[1];
    return Some((game_id, game_sessions));
}

fn valid_game(row: &str) -> Option<(&str, i32)> {
    if let Some((game_id, game_sessions)) = get_game_info(row) {
        let row_valid = validate_game_sessions(game_sessions);
        Some((game_id, row_valid))
    } else {
        return None;
    }
}

fn validate_game_sessions(game_sessions: &str) -> i32 {
    for session in game_sessions.split(';') {
        let session_colours = process_game_session(session);
        if session_colours["red"] > 12
            || session_colours["green"] > 13
            || session_colours["blue"] > 14
        {
            return 0;
        }
    }
    1
}

fn process_game_session(session: &str) -> HashMap<&str, i32> {
    let mut session_colours = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);
    for colour_amounts in session.split(',') {
        let colour_amount = colour_amounts.trim().split(' ').collect::<Vec<&str>>();
        if colour_amount.len() != 2 {
            continue;
        }
        let amount = match colour_amount[0].parse::<i32>() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let colour = colour_amount[1];
        if let Some(session_colour_amount) = session_colours.get_mut(colour) {
            *session_colour_amount += amount;
        }
    }
    session_colours
}
