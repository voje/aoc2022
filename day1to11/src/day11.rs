pub mod simulation;
pub mod monkey;

use simulation::Simulation;

pub fn part1(data: &str) -> u128 {
    let relief = true;
    let mut sim = Simulation::new(relief); 
    sim.parse_input(data);

    let n_rounds = 20;
    sim.run(n_rounds);
    sim.top2_result()
}

pub fn part2(data: &str) -> u128 {
    let relief = false;
    let mut sim = Simulation::new(relief); 
    sim.parse_input(data);

    let n_rounds = 21;
    sim.run(n_rounds);
    sim.top2_result()
}

#[test]
fn day11_part1() {
    assert_eq!(part1(DATA), 10605);
}

#[test]
fn day11_keeping_worries_low() {
    let relief = false;
    let mut sim = Simulation::new(relief); 
    sim.parse_input(DATA);

    let n_rounds = 20;
    sim.run(n_rounds);
    assert_eq!(sim.get_insp_counts(), vec![99,97,8,103]);
}

/*
#[test]
fn day11_part2() {
    assert_eq!(part2(DATA), 2713310158);
}
*/

#[cfg(test)]
const DATA: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97\r
  Operation: new = old * old\r
  Test: divisible by 13\r
    If true: throw to monkey 1\r
    If false: throw to monkey 3\r

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
