
enum Direction {
    Up,
    Down,
    Left,
    Right
}

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

    // Return a vector with a line of sight from a tree
    // 12345
    // 65432
    // 34567
    // 54321
    // get_line(1, 2, 
    fn get_line(&self, y: usize, x: usize, d: Direction) -> Vec<u32> {
        let mut res: Vec<u32> = vec![];
        match d {
            Direction::Up => {
                for yy in (0..y).rev() {
                    res.push(self.g[yy][x]);
                }
            }
            Direction::Down => {
                for yy in (y+1)..self.h {
                    res.push(self.g[yy][x]);
                }
            }
            Direction::Left => {
                for xx in (0..x).rev() {
                    res.push(self.g[y][xx]);
                }
            }
            Direction::Right => {
                for xx in (x+1)..self.w {
                    res.push(self.g[y][xx]);
                }
            }
        }
        res
    }

    // pick tree, go l,r,up,down from tree, see if we encounter
    // same size or bigger tree
    fn tree_is_visible(&self, y: usize, x: usize) -> bool {
        let tree = self.g[y][x];

        let right = self.get_line(y, x, Direction::Right);
        let visible_from_right = p1_clear_view(tree, right);

        let left = self.get_line(y, x, Direction::Left);
        let visible_from_left = p1_clear_view(tree, left);

        let up = self.get_line(y, x, Direction::Up);
        let visible_from_up = p1_clear_view(tree, up);

        let down = self.get_line(y, x, Direction::Down);
        let visible_from_down = p1_clear_view(tree, down);

        visible_from_left ||
        visible_from_right ||
        visible_from_up ||
        visible_from_down
    }

    // pick tree, go l,r,up,down from tree, see if we encounter
    // same size or bigger tree
    fn tree_scenic_score(&self, y: usize, x: usize) -> u32 {
        let tree = self.g[y][x];

        let right = self.get_line(y, x, Direction::Right);
        let ss_right = line_scenic_score(tree, right);

        let left = self.get_line(y, x, Direction::Left);
        let ss_left = line_scenic_score(tree, left);

        let up = self.get_line(y, x, Direction::Up);
        let ss_up = line_scenic_score(tree, up);

        let down = self.get_line(y, x, Direction::Down);
        let ss_down = line_scenic_score(tree, down);

        ss_right * ss_left * ss_up * ss_down
    }

    // very dumb and ugly O(n2) solution
    fn visible_trees(&self) -> u32 {
        let mut vis_mask = vec![vec![false; self.w] ;self.h]; 

        for y in 0..self.g.len() {
            for x in 0..self.g[0].len() {
                // println!("y: {}, x: {}", y, x);
                vis_mask[y][x] = self.tree_is_visible(y, x);
            }
        }

        // print_grid(&self.g);
        // print_mask(&vis_mask);
        count_trues(&vis_mask)
    }

    // very dumb and ugly O(n2) solution
    fn scenic_scores(&self) -> u32 {
        let mut ss = vec![vec![0; self.w] ;self.h]; 

        for y in 0..self.g.len() {
            for x in 0..self.g[0].len() {
                // println!("y: {}, x: {}", y, x);
                ss[y][x] = self.tree_scenic_score(y, x);
            }
        }

        // print_grid(&self.g);
        // print_grid(&ss);
        find_max(&ss)
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

fn print_grid(m: &Vec<Vec<u32>>) {
    let mut s = String::new();
    for row in m {
        for el in row {
            s.push_str(&format!("{}", el).to_string());
        }
        s.push_str("\n");
    }
    println!("{}", s);
}


fn p1_clear_view(tree: u32, line: Vec<u32>) -> bool {
    for t in line {
        if t >= tree {
            return false;
        }
    }
    true
}

fn count_trues(m: &Vec<Vec<bool>>) -> u32 {
    let mut c = 0;
    for row in m {
        for b in row {
            if *b { c += 1 };
        }
    }
    c
}

fn line_scenic_score(tree: u32, line: Vec<u32>) -> u32 {
    let mut ss = 0;
    for t in line {
        ss += 1;
        if t >= tree {
            break;
        }
    }
    ss 
}

fn find_max(m: &Vec<Vec<u32>>) -> u32 {
    let mut max = 0;
    for row in m {
        for el in row {
            if el > &max {
                max = *el;
            }
        }
    }
    max
}

pub fn part1(data: &str) -> u32 {
    let g = Grid::new(data);
    g.visible_trees()
}

pub fn part2(data: &str) -> u32 {
    let g = Grid::new(data);
    g.scenic_scores()
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

#[test]
fn day08_part2() {
    assert_eq!(part2(DATA), 8);
}

