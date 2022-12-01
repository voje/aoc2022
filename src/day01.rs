pub fn top_dwarves(mut v: Vec<i32>) -> (i32, i32, i32) {
    let first = v.pop().unwrap(); 
    let second = v.pop().unwrap(); 
    let third = v.pop().unwrap(); 

    (first, second, third)
}

pub fn aggregate_calories(s: &str) -> Vec<i32> {
    let mut dwarves: Vec<i32> = vec![]; 

    let mut current_dwarf_acc = 0;
    for line in s.lines() {
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
    dwarves
}

#[test]
fn dwarves() {
    const DATA: &str ="
    1000
    2000
    3000

    4000

    5000
    6000


    7000
    8000
    9000

    10000
    ";

    let dwarves = aggregate_calories(DATA); 
    let (first, second, third) = top_dwarves(dwarves);

    assert_eq!(first, 24000);
    assert_eq!(first + second + third, 45000);
}
