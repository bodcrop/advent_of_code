use std::{fs, collections::LinkedList};

#[derive(Clone)]
struct SupplyStack {
    column_number: usize,
    crates: LinkedList<String>,
}

impl SupplyStack {
    fn print_top_level(&self) {
        let top_item = self.crates.front().expect("stack is empty");
        let str_column_number = self.column_number.to_string();
        println!("For column: {str_column_number} top item is {top_item}");
    }

    fn print_stack(&self) {
        let str_column_number = self.column_number.to_string();
        println!("For column: {str_column_number} the items are:");
        for crate_name in &self.crates {
            print!("{crate_name} \n");
        }
        print!("\n \n")

    }
    fn init_crate(&mut self,crate_name: &str) {
        // for the initialization items is read top down
        self.crates.push_back(crate_name.to_string());
    }
    fn add_crate(&mut self,crate_name: &str) {
        self.crates.push_front(crate_name.to_string());
    }
    fn remove_top_crate(&mut self) -> Option<String> {
        return self.crates.pop_front()
    }
}
fn main() {
    println!("Hello, world!");
    let content: String = fs::read_to_string("src/input.txt").expect("could not read String");
    let parts: Vec<&str> = content.split("\n\n").collect();
    let start_stack: Vec<SupplyStack> = initialize_start_stack(parts[0]);
    
    let mut item_stack_q1 = start_stack.clone();
    do_stack_rearanges(parts[1],&mut item_stack_q1,move_n_items_crate_mover_9000);
    print!("Answer question 1 \n");
    for column_stack in item_stack_q1 {
        column_stack.print_top_level();
    }


    let mut item_stack_q2 = start_stack.clone();
    do_stack_rearanges(parts[1],&mut item_stack_q2,move_n_items_crate_mover_9001);
    print!("Answer question 2 \n");
    for column_stack in item_stack_q2 {
        column_stack.print_top_level();
    }

}

fn extract_value_from_brackets(input: &str) -> Option<&str> {
    // println!("{input}");
    if input.starts_with('[') && input.ends_with(']') {
        let inner_value = &input[1..input.len() - 1];
        Some (inner_value)
    } else {
        None
    }
}

fn split_string_by_n_chars(input: &str, n: usize) -> Vec<String> {
    return input.chars()
         .collect::<Vec<_>>()
         .chunks(n)
         .map(|chunk| chunk.iter().collect())
         .collect()
}

fn initialize_start_stack(start_stack_string: &str) -> Vec<SupplyStack> {
    let mut start_stack: Vec<SupplyStack> = Vec::new();
    let stack_rows: Vec<&str> = start_stack_string.split("\n").collect();
    for i in 0..9 {
        let crates: LinkedList<String> = LinkedList::new();
        let column_number:usize = i;
        start_stack.push(SupplyStack{crates,column_number});
    }
    for row in stack_rows {
        for (stack_column ,content) in split_string_by_n_chars(row.trim(),4).iter().enumerate() {
            match extract_value_from_brackets(content.trim()) {
                Some(extracted_value) => {
                    start_stack[stack_column].init_crate(extracted_value)
                }
                None => {
                    ()
                }
            }
        }
    }

    print!("start stack \n");
    for column_stack in &start_stack {
        column_stack.print_stack();
    }
    print!("Top of start stack \n");
    for column_stack in &start_stack {
        column_stack.print_top_level();
    }

    return start_stack;
}

fn do_stack_rearanges(stack_permutations:&str,item_stack:&mut Vec<SupplyStack>,move_items_using_crate: fn(&mut Vec<SupplyStack>,u32,usize,usize)) {
    let rearrangements: Vec<&str> = stack_permutations.split("\n").collect();
    for rearrangement in rearrangements {
        let orders: Vec<&str> = rearrangement.split(" ").collect();
        let n_items: u32 = orders[1].parse().expect("Invalid amount");
        let source: usize = orders[3].parse().expect( "Invalid 'to' index");
        let destination: usize = orders[5].parse().expect("Invalid 'to' index");
        move_items_using_crate(item_stack,n_items,source,destination);
    }

}

fn move_n_items_crate_mover_9000(item_stack:&mut Vec<SupplyStack>, n_items: u32,source: usize,destination: usize) {
    // println!("Taking {n_items} from {source} to {destination}");
    let source_index = source -1;
    let destination_index: usize = destination -1;
    for _i in 0..n_items {
        let item = (*item_stack)[source_index].remove_top_crate().expect("Could not take items from stack, invalid rearrangment!.");
        (*item_stack)[destination_index].add_crate(&item);
    }
}

fn move_n_items_crate_mover_9001(item_stack:&mut Vec<SupplyStack>, n_items: u32,source: usize,destination: usize) {
    let source_index = source -1;
    let destination_index: usize = destination -1;
    let mut crates_moved: Vec<String> = Vec::new();
    for _i in 0..n_items {
        let item = (*item_stack)[source_index].remove_top_crate().expect("Could not take items from stack, invalid rearrangment!.");
        crates_moved.push(item)
    }
    for crate_name in crates_moved.iter().rev() {
        (*item_stack)[destination_index].add_crate(&crate_name);
    }

}