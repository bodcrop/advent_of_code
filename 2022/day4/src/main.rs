use std::fs;
use std::collections::HashSet;
use std::cmp;
struct Section {
    start: u32,
    end: u32,
}

fn parse_section(section_str: &str) -> Section {
    let parts: Vec<&str> = section_str.split("-").collect();
    let start: u32 = parts[0].parse().expect("Invalid could not parse {parts[0]}");
    let end: u32 = parts[1].parse().expect("Invalid could not parse {parts[1]}");
    return Section{start,end};
}
fn main() {
    let content = fs::read_to_string("src/input.txt").expect("Error could not read the file");
    let pairs: Vec<&str> = content.split("\n").collect();
    let mut number_completely_overlapping: u32 = 0;
    let mut number_partial_overlapping: u32 = 0;
    for pair in pairs {
        let mut sections_first_elf: HashSet<u32> = HashSet::new();
        let mut sections_second_elf: HashSet<u32> = HashSet::new();

        let sections: Vec<Section> = pair.split(",").map(|interval_str| parse_section(interval_str.trim())).collect();
        for num in sections[0].start..sections[0].end+1 {
            sections_first_elf.insert(num);
        }

        for num in sections[1].start..sections[1].end+1 {
            sections_second_elf.insert(num);
        }

        let all_sections: HashSet<&u32> = sections_first_elf.union(&sections_second_elf).collect();
        if all_sections.len() == cmp::max(sections_first_elf.len(),sections_second_elf.len()) {
            number_completely_overlapping += 1;
        }

        if all_sections.len() < sections_first_elf.len() + sections_second_elf.len() {
            number_partial_overlapping += 1;
        }


    }

    println!("Number of fully containting pairs equal {number_completely_overlapping} ");
    println!("Number of partial overlapping pairs equal {number_partial_overlapping} ");
}
