use std::str::FromStr;
use std::num::ParseIntError;

// Point
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        Ok(Point {
            x: coords[0].parse::<i32>()?,
            y: coords[1].parse::<i32>()?,
        })
    }
}


// Line
#[derive(Debug)]
pub struct Line (pub Point, pub Point);

impl Line {
    pub fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        if self.horizontal() {
            let small = std::cmp::min(self.0.y, self.1.y);
            let big = std::cmp::max(self.0.y, self.1.y);

            for i in small..big+1 {
                points.push(Point{x: self.0.x, y: i})
            }
        }
        else if self.vertical() {
            let small = std::cmp::min(self.0.x, self.1.x);
            let big = std::cmp::max(self.0.x, self.1.x);

            for i in small..big+1 {
                points.push(Point{x: i, y: self.0.y})
            }
        }
        else {
            let small = std::cmp::min_by(&self.0, &self.1, |p1, p2| p1.x.cmp(&p2.x));
            let big = std::cmp::max_by(&self.0, &self.1, |p1, p2| p1.x.cmp(&p2.x));

            let diff_x = big.x - small.x;
            let diff_y = big.y - small.y;

            for i in 0..diff_x+1 {
                points.push(Point{
                    x: small.x + (i * diff_x.signum()),
                    y: small.y + (i * diff_y.signum()),
                })
            }
        }

        points
    }

    pub fn horizontal(&self) -> bool {
        self.0.x == self.1.x
    }

    pub fn vertical(&self) -> bool {
        self.0.y == self.1.y
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
