use std::collections::HashMap;

fn map_weapon(w: &str) -> &str {
    let m1: HashMap<&str, &str> = HashMap::from([
        ("A", "rock"),
        ("B", "paper"),
        ("C", "scissors"),
        ("X", "rock"),
        ("Y", "paper"),
        ("Z", "scissors"),
    ]);
    m1[w]
}

fn oracle(opponents_weapon: &str, outcome: &str) -> String {
    let rps = vec!["rock", "paper", "scissors"]; 

    let opp_idx = rps.iter().position(|&x| x == opponents_weapon).unwrap();
    // println!("{}", opp_idx);

    let lose = "X";
    let draw = "Y";
    let _win = "Z";
    if outcome == lose {
        return rps[(opp_idx + 2) % 3].to_owned()
    }
    else if outcome == draw {
        return opponents_weapon.to_string()
    }
    else {
        return rps[(opp_idx + 1) % 3].to_owned()
    }
}

// Calculate player_b's score
pub fn score_fight(a: &str, b: &str) -> i64 {
    let m2: HashMap<&str, i64> = HashMap::from([
        ("rock", 1),
        ("paper", 2),
        ("scissors", 3),
    ]);

    let score = m2[&b];

    // Draw
    if a == b {
        return score + 3
    }

    // Won
    if (b == "rock" && a == "scissors") ||
        (b == "paper" && a == "rock") ||
        (b == "scissors" && a == "paper") {
            return score + 6
        }
    
    // Lost
    return score
}

pub fn score_fight_1(opponents_weapon: &str, desired_outcome: &str) -> i64 {
    let my_weapon = oracle(opponents_weapon, desired_outcome);
    // println!("{}, {}, {}", opponents_weapon, desired_outcome, my_weapon);
    score_fight(opponents_weapon, &my_weapon)
}

pub fn tournament(duels: &str) -> (i64, i64) {
    let mut total_score = 0;
    let mut total_score_1 = 0;
    for duel in duels.lines() {
        let spl: Vec<&str> = duel.split(" ").collect(); 
        let a = map_weapon(spl[0]);
        let b = map_weapon(spl[1]);
        total_score += score_fight(a, b);
        total_score_1 += score_fight_1(a, spl[1]);
    }
    (total_score, total_score_1)
}

#[cfg(test)]
const DATA: &str = "A Y
B X
C Z";

#[test]
fn day02_1() {
    let (s, _) = tournament(DATA); 
    assert_eq!(s, 15);
}

#[test]
fn day02_2() {
    let (_, s1) = tournament(DATA); 
    assert_eq!(s1, 12);
}

