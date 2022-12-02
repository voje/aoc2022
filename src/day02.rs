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

pub fn tournament(duels: &str) -> i64 {
    let mut total_score = 0;
    for duel in duels.lines() {
        let spl: Vec<&str> = duel.split(" ").collect(); 
        let a = map_weapon(spl[0]);
        let b = map_weapon(spl[1]);
        let s = score_fight(a, b);
        // println!("{} vs {} => {}", a, b, s);
        total_score += s;
    }
    total_score
}


#[test]
fn game0() {
    const DATA: &str = "A Y
B X
C Z";
    
    assert_eq!(tournament(DATA), 15);
}
