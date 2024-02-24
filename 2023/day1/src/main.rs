use std::collections::HashMap;
use std::{fs, vec};
fn main() {
    println!("Hello, world!");
    let content: String = fs::read_to_string("src/input.txt").expect("Could not read input file");
    let rows: Vec<&str> = content.split("\n").collect();
    q1(&rows);
    q2(&rows);
}

fn q1(rows: &Vec<&str>) {
    let mut calibration_sum_q1: u32 = 0;
    for row in rows {
        let mut row_numberical_digits: Vec<u32> = Vec::new();
        for row_char in row.chars() {
            if row_char.is_digit(10) {
                row_numberical_digits.push(row_char.to_digit(10).unwrap());
            }
        }
        let number_of_digits = row_numberical_digits.len();
        if number_of_digits == 0 {
            println!("No digits found for {}", row)
        } else {
            calibration_sum_q1 +=
                10 * row_numberical_digits[0] + row_numberical_digits[number_of_digits - 1];
        }
    }
    println!("Q1: The total sum is {}", calibration_sum_q1);
}

fn q2(rows: &Vec<&str>) {
    let string_scalar_mapping = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let mut string_or_digit_sum = 0;

    for row in rows {
        let mut index_value = Vec::new();
        for key in string_scalar_mapping.keys() {
            let l_index = row.find(key);
            if l_index != None {
                index_value.push((l_index.unwrap(), string_scalar_mapping[key]))
            }

            let r_index = row.rfind(key);
            if r_index != None {
                index_value.push((r_index.unwrap(), string_scalar_mapping[key]))
            }
        }
        index_value.sort_by(|a, b| a.0.cmp(&b.0));

        if index_value.len() > 0 {
            string_or_digit_sum += 10 * index_value[0].1 + index_value[index_value.len() - 1].1
        } else {
            println!("Could not find numberic value in {row}")
        }
    }
    println!("Q2: the total sum of numeric and string representations is: {string_or_digit_sum}")
}
