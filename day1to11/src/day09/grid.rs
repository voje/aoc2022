#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    // Optional field, carrying value, ignored in arithmetic operations
    pub val: Option<i32>,
}

impl Point {
    pub fn new(x: i32, y: i32, val: Option<i32>) -> Point {
        Point {x, y, val}
    }

    // While Add/Sub craete another object, move_pt simply updates 
    // the object's coordinates
    pub fn move_point(&mut self, p: &Point) {
        self.x = self.x + p.x;
        self.y = self.y + p.y;
    }

    // Return a vector spanning between self and other
    pub fn vec_to(&self, other: &Point) -> Point {
        Point {
            x: other.x - self.x,
            y: other.y - self.y,
            val: None,
        }
    }

    // Return the unit vector
    pub fn unit(&self) -> Point {
        let mut x = 0;
        let mut y = 0;
        if self.x != 0 {
            x = self.x / self.x.abs();
        }
        if self.y != 0 {
            y = self.y / self.y.abs();
        }
        Point {
            x,
            y,
            val: None,
        }
    }

}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y
    }
}

#[derive(Debug)]
pub struct Grid {
    y_max: i32,
    y_min: i32,
    x_max: i32,
    x_min: i32,
    empty_field: char,
    empty_point_placeholder: char,
}

impl Grid {
    pub fn new(radius: i32) -> Grid {
        Grid {
            y_max: radius,
            y_min: -radius,
            x_max: radius,
            x_min: -radius,
            empty_field: '.',
            empty_point_placeholder: 'x',
        }
    }

    pub fn draw(&self, points: &Vec<Point>) -> String {
        let mut out = String::new();

        for y in self.y_min..self.y_max {
            for x in self.x_min..self.x_max {
                let p = Point::new(x, y, None);
                let mut p_str = format!("{} ", self.empty_field);
                for pp in points {
                    if *pp == p {
                        let val = match pp.val {
                            Some(v) => v,
                            None => self.empty_point_placeholder as i32,
                        };
                        p_str = format!("{} ", val);
                        break;
                    }
                }
                out.push_str(&p_str);
            }
            out.push_str("\n");
        }
        out
    }
}

/*
#[test]
fn day09_grid_1() {
    let g = Grid {
        y_max: 5,
        y_min: -5,
        x_max: 5,
        x_min: -5,
        empty_field: '.',
        empty_point_placeholder: 'x',
    };
    let points = vec![
        Point::new(0,0,Some(0)),
        Point::new(0,1,Some(1)),
        Point::new(0,2,Some(2)),
        Point::new(0,3,Some(3)),
        Point::new(-5,-5,Some(4)),
        Point::new(-4,-4,Some(5)),
    ];
    println!("{}", g.draw(&points));
}
*/

