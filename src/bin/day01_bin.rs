use std::fs;

const FILE_PATH: &str = "./inputs/day01_1.txt";
// const MAX_DWARF_ANSW: i32 = 69310;
// const MAX_THREE_ANSW: i32 = 206104;

fn main() {
    println!("Day 1 baby!");

    let contents = fs::read_to_string(FILE_PATH).unwrap();
    let mut dwarves: Vec<i32> = vec![];

    let mut current_dwarf_acc = 0;
    for line in contents.lines() {
        if line.len() == 0 {
            dwarves.push(current_dwarf_acc);
            current_dwarf_acc = 0;  
            continue;
        }
        let int_rep = line.parse::<i32>().unwrap();
        current_dwarf_acc += int_rep;
    }
    // Add last dwarf
    dwarves.push(current_dwarf_acc);
    
    dwarves.sort();

    let max_dwarf = dwarves.pop().unwrap();
    let runner_up = dwarves.pop().unwrap();
    let third_place = dwarves.pop().unwrap();
    let total = max_dwarf + runner_up + third_place;

    println!("Calorie boii: {}", max_dwarf);
    println!("Max boys: {}", total);
}
