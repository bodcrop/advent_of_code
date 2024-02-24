use std::fs;
use std::collections::HashMap;



fn main() {
    println!("Hello, world!");
    let content = fs::read_to_string("src/input.txt").expect("Error while reading input");
    let score_for_shape: HashMap<&str, i32> = HashMap::from([
        ("X",1),
        ("Y",2),
        ("Z",3)
        ]);
    let score_for_move : HashMap<&str,i32> = HashMap::from([
        ("A X",3),
        ("A Y",6),
        ("A Z",0),

        ("B X",0),
        ("B Y",3),
        ("B Z",6),

        ("C X",6),
        ("C Y",0),
        ("C Z",3),
    ]);

    let score_for_strategy_2 : HashMap<&str,i32> = HashMap::from([
        ("A X",3),
        ("B X",1),
        ("C X",2),

        ("A Y",4),
        ("B Y",5),
        ("C Y",6),


        ("A Z",8),
        ("B Z",9),
        ("C Z",7),

    ]);


    let mut total_score_round_1: i32 = 0;
    let mut total_score_round_2: i32 = 0;
    for line in content.split("\n") {
        let game_move: Vec<&str> = line.split_whitespace().collect();
        let your_move: &str = game_move[1];
        let round_score: i32 = score_for_shape.get(your_move).unwrap() + score_for_move.get(line).unwrap();
        total_score_round_1 += round_score;
        total_score_round_2 += score_for_strategy_2.get(line).unwrap();
        }

    println!("Total score question1: {total_score_round_1}");
    println!("Total score question2: {total_score_round_2}");
}