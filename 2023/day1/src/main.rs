use std::fs;

fn main() {
    println!("Hello, world!");
    let content: String = fs::read_to_string("src/input.txt").expect("Could not read input file");
    let rows = content.split("\n");
    let mut calibration_sum_q1: u32 = 0;
    for row in rows {
        let mut row_numberical_digits: Vec<u32> = Vec::new();
        for letter in row.chars() {
            if letter.is_digit(10) {
                row_numberical_digits.push(letter.to_digit(10).unwrap());
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
    println!("Q1: The total sum is {}", calibration_sum_q1)
}
