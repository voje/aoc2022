
#[derive(Debug)]
struct Grid {
    g: Vec<Vec<u32>>,
    w: usize,
    h: usize,
}

impl Grid {
    fn new(data: &str) -> Grid {
        const RADIX: u32 = 10;
        let mut g = Grid {
            g: vec![],
            w: 0,
            h: 0,
        };
        for line in data.lines() {
            // let row = vec![]
            g.g.push(line
                .chars()
                .map(|c| c.to_digit(RADIX).unwrap())
                .collect()
            );
        }
        g.w = g.g.len();
        g.h = g.g[0].len();
        g
    }

    fn print_grid(&self) {
        let mut s = String::new();
        for row in &self.g {
            for b in row {
                s.push_str(&format!("{}", b).to_string());
            }
            s.push_str("\n");
        }
        println!("{}", s);
    }

    // pick tree, go l,r,up,down from tree, see if we encounter
    // same size or bigger tree
    fn tree_is_visible(&self, y: usize, x: usize) -> bool {
        let tree = self.g[y][x];

        let mut from_left = true;
        for xx in 0..x {
            if self.g[y][xx] >= tree {
                from_left = false;
                break;
            }
        }

        let mut from_right = true;
        for xx in (x + 1)..self.w {
            if self.g[y][xx] >= tree {
                from_right = false;
                break;
            }
        }

        let mut from_up = true;
        for yy in 0..y {
            if self.g[yy][x] >= tree {
                from_up = false;
                break;
            }
        }

        let mut from_down = true;
        for yy in (y + 1)..self.h {
            if self.g[yy][x] >= tree {
                from_down = false;
                break;
            }
        }

        // For testing
        // from_left = false;
        // from_right = false;
        // from_up = false;
        // from_down = false;

        return from_left || from_right || from_up || from_down;
    }

    // very dumb and ugly O(n2) solution
    fn visible_trees(&self) -> i64 {
        let h = self.g.len();
        let w = self.g[0].len();
        let mut vis_mask = vec![vec![false; w] ;h]; 

        for y in 0..self.g.len() {
            for x in 0..self.g[0].len() {
                // println!("y: {}, x: {}", y, x);
                vis_mask[y][x] = self.tree_is_visible(y, x);
            }
        }

        // self.print_grid();
        // print_mask(&vis_mask);
        count_trues(&vis_mask)
    }
}

fn print_mask(m: &Vec<Vec<bool>>) {
    let mut s = String::new();
    for row in m {
        for b in row {
            if *b {
                s.push_str("1");
            } else {
                s.push_str("0");
            }
        }
        s.push_str("\n");
    }
    println!("{}", s);
}

fn count_trues(m: &Vec<Vec<bool>>) -> i64 {
    let mut c = 0;
    for row in m {
        for b in row {
            if *b { c += 1 };
        }
    }
    c
}

pub fn part1(data: &str) -> i64{
    let g = Grid::new(data);
    g.visible_trees()
}

#[cfg(test)]
const DATA: &str = "30373
25512
65332
33549
35390";

#[test]
fn day08_grid() {
    let g = Grid::new(DATA);
    println!("{:?}", g);
}

#[test]
fn day08_part1() {
    assert_eq!(part1(DATA), 21);
}
