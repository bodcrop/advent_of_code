use std::fs;

fn main() {
    println!("Hello, world!");
    let file_path = "./src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let mut sum_calories_each_elf = Vec::new();

    for bag_each_elf in contents.split("\n\n") {
        let calories_elf = bag_each_elf.split("\n");
        let sum: u32 = calories_elf.map(|s| s.parse::<u32>().expect("parse error")).sum();
        sum_calories_each_elf.push(sum)
    }

    sum_calories_each_elf.sort();
    sum_calories_each_elf.reverse();
    
    println!( "Most calories equals: {}", sum_calories_each_elf[0] );
    let sum_three_largest = &sum_calories_each_elf[..3].iter().sum::<u32>();
    println!("Top three most colories equals {:?}",sum_three_largest);
    
}
