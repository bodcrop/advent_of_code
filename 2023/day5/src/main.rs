use std::fs;
fn main() {
    let content: String = fs::read_to_string("src/input.txt").expect("Could not read input file");
    let different_parts: Vec<&str> = content.split("\n\n").collect();

    //initialise seeds
    let mut seeds: Vec<i64> = different_parts[0].split(": ").collect::<Vec<&str>>()[1]
        .split(" ")
        .map(|seed| {
            seed.parse::<i64>().unwrap_or_else(|_| {
                println!("Failed to parse: {}", seed);
                0 // or any default value you want
            })
        })
        .collect();
    for seed in seeds.iter_mut() {
        // println!("\n \n For seed {seed}:");
        for part in &different_parts[1..different_parts.len()] {
            let mut new_seed = *seed;
            for (i, row) in part.split("\n").enumerate() {
                if row.len() < 3 {
                    break;
                }
                if i == 0 {
                    // println!("{row}");
                    continue;
                }

                let mapper: Vec<i64> = row
                    .split(" ")
                    .map(|part| part.parse::<i64>().unwrap())
                    .collect();

                if mapper[1] <= *seed && *seed <= mapper[1] + mapper[2] {
                    new_seed = *seed - mapper[1] + mapper[0];
                }
            }
            // println!("{seed} get mapped to {new_seed}");
            *seed = new_seed;
        }
    }

    let min_value = seeds.iter().min().unwrap();
    println!("Q1: The minum value is {min_value}");
}
