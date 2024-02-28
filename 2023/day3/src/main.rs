use regex::Regex;
use std::cmp;
use std::fs;

//TODO how to handle newlines as a character?
fn main() {
    println!("Hello, world!");
    let content = fs::read_to_string("src/input.txt").expect("Could not read input file");
    println!("{content}");
    let content_rows = content.split("\n").collect::<Vec<&str>>();

    let max_rows = content_rows.len();
    let max_num_charcters = content_rows[0].len();
    println!("max charact {max_num_charcters}");

    let regex_digits: Regex = Regex::new(r"\d+").unwrap();
    let regex_only_dots: Regex = Regex::new(r"^[.]*$").unwrap();
    let mut sum_number = 0;
    let mut y_coord = 0;
    for row in &content_rows {
        for number_location in regex_digits.find_iter(row) {
            let number = row
                .get(number_location.0..number_location.1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let x_coord_start: usize = match number_location.0 {
                0 => 0,
                _ => (number_location.0) - 1,
            };
            let x_coord_end: usize = cmp::min((number_location.1) + 1, max_num_charcters);
            let mut no_symbol_seen = true;
            // println!(
            //     "{number} X_Start{} -> {x_coord_start}, X_end {} -> {x_coord_end}, Y {y_coord}",
            //     { number_location.0 },
            //     { number_location.1 }
            // );
            // top row
            if y_coord != 0 {
                no_symbol_seen = no_symbol_seen
                    && regex_only_dots.is_match(
                        content_rows[y_coord - 1]
                            .get(x_coord_start..x_coord_end)
                            .unwrap_or(".."),
                    );
            }
            // left
            no_symbol_seen = no_symbol_seen
                && regex_only_dots.is_match(
                    content_rows[y_coord]
                        .get(x_coord_start..number_location.0)
                        .unwrap_or(".."),
                );

            //right row
            let temp = content_rows[y_coord]
                .get(number_location.1..x_coord_end)
                .unwrap_or("..");
            no_symbol_seen = no_symbol_seen
                && regex_only_dots.is_match(
                    content_rows[y_coord]
                        .get(number_location.1..x_coord_end)
                        .unwrap_or(".."),
                );

            //bottom
            if y_coord < max_rows {
                no_symbol_seen = no_symbol_seen
                    && regex_only_dots.is_match(
                        content_rows[y_coord + 1]
                            .get(x_coord_start..x_coord_end)
                            .unwrap_or(".."),
                    );
            }
            if !no_symbol_seen {
                sum_number += number;
            } else {
                // println!("Could not see neighbours of number {number} for coords {x_coord_start} {x_coord_end}, {y_coord} ")
            }

            if y_coord == max_rows {
                break;
            }
        }
        y_coord += 1;
    }

    println!("Total sum equals {sum_number}")
}
