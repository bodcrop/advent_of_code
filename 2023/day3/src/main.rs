use regex::Regex;
use std::cmp;
use std::fs;
use std::usize;

//TODO how to handle newlines as a character?
fn main() {
    let content = fs::read_to_string("src/test.txt").expect("Could not read input file");
    let content_rows: Vec<&str> = content.split("\n").collect::<Vec<&str>>();
    // q1(&content_rows);
    q2(&content_rows);
}

fn q1(content_rows: &Vec<&str>) {
    let max_rows = content_rows.len();
    let max_num_charcters = content_rows[0].len();
    println!("max charact {max_num_charcters}");

    let regex_digits: Regex = Regex::new(r"\d+").unwrap();
    let regex_only_dots: Regex = Regex::new(r"^[.]*$").unwrap();
    let mut sum_number = 0;
    let mut y_coord = 0;
    for row in content_rows {
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

            //top
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

fn q2(content_rows: &Vec<&str>) {
    let r_find_gear: Regex = Regex::new(r"\*").unwrap();
    let r_find_digit: Regex = Regex::new(r"\d+").unwrap();

    let mut total_score_v2: i32 = 0;
    let max_rows = content_rows.len();
    let max_num_charcters = content_rows[0].len();

    for (y, row) in content_rows.iter().enumerate() {
        for x_gear_location in row.match_indices("*").map(|(i, _)| i) {
            let x_coord_start: usize = match x_gear_location {
                0 => 0,
                _ => (x_gear_location) - 1,
            };
            let x_coord_end: usize = cmp::min(x_gear_location + 2, max_num_charcters);
            let mut found_number_neighbours: Vec<(usize, usize, usize)> = Vec::new();

            //check_top
            if y > 0 {
                let mut numbers_coords_top: Vec<(usize, usize)> = r_find_digit
                    .find_iter(
                        content_rows[y - 1]
                            .get(x_coord_start..x_coord_end)
                            .unwrap_or(".."),
                    )
                    .collect();
                for number in numbers_coords_top {
                    found_number_neighbours.push((
                        x_coord_start + number.0,
                        x_coord_start + number.1,
                        y - 1,
                    ))
                }
            }

            //check_current row
            let mut numbers_coords: Vec<(usize, usize)> = r_find_digit
                .find_iter(
                    content_rows[y]
                        .get(x_coord_start..x_coord_end)
                        .unwrap_or(".."),
                )
                .collect();
            for number in numbers_coords {
                found_number_neighbours.push((
                    x_coord_start + number.0,
                    x_coord_start + number.1,
                    y,
                ))
            }

            //check_bottom
            if y < max_rows {
                let mut number_coords_bottom: Vec<(usize, usize)> = r_find_digit
                    .find_iter(
                        content_rows[y + 1]
                            .get(x_coord_start..x_coord_end)
                            .unwrap_or(".."),
                    )
                    .collect();
                for number in number_coords_bottom {
                    found_number_neighbours.push((
                        x_coord_start + number.0,
                        x_coord_start + number.1,
                        y + 1,
                    ))
                }
            }

            if found_number_neighbours.len() > 1 {
                println!("Found * on location: {x_gear_location} on row {y}");
                for neighbour in found_number_neighbours {
                    println!(
                        "Found number {} as a neighbour",
                        content_rows[neighbour.2]
                            .get(neighbour.0..neighbour.1)
                            .unwrap()
                    )
                }
            }
        }
    }
}
fn get_compelete_number(start_location: usize, end_location: usize, row: &&str) -> i32 {
    return 42;
}
