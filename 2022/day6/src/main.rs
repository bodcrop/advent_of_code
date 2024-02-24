use std::{fs};
fn main() {
    println!("Hello, world!");
    let content: String = fs::read_to_string("src/input.txt").expect("Could not read input file");
    println!("{content}");
    get_start_of_message(&content,4);
    get_start_of_message(&content, 14)
 
}

fn get_start_of_message(content: &str,n_unique_messages: usize) {
    let mut start_of_packet_marker: u32 = 0;
    let mut unique_characters_receiveced= Vec::new();
    for character in content.chars() {
        start_of_packet_marker += 1;
        if unique_characters_receiveced.contains(&character) { 
            let index = unique_characters_receiveced.iter().position(|&r| r == character).unwrap();
            unique_characters_receiveced.push(character);
            unique_characters_receiveced.drain(0..index+1);
        } else {
            unique_characters_receiveced.push(character);
        }
        if unique_characters_receiveced.len() == n_unique_messages {
            println!("{:?}",unique_characters_receiveced);
            println!("Start of packet marker at {start_of_packet_marker} after {:?} unique messages", n_unique_messages);
            break;
        }
    }
}
