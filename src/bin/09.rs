use std::collections::HashSet;

use aoc2022::file_io::lines_in_file;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {

    fn zero() -> Point {
        Point { x: 0, y: 0 }
    }

    fn plus(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn minus(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn is_far_from(&self, other: &Point) -> bool {
        let difference = self.minus(other);
        difference.x.abs() >= 2 || difference.y.abs() >= 2
    }

    fn get_closest_touching(&self, other: &Point) -> Point {
        let difference = self.minus(other);
        other.plus(&Point {
            x: difference.x.signum() * (difference.x != 0) as i32,
            y: difference.y.signum() * (difference.y != 0) as i32,
        })
    }
}

fn get_increment(direction: &str) -> Point {
    match direction {
        "U" => Point { x: 0, y: 1 },
        "D" => Point { x: 0, y: -1 },
        "L" => Point { x: -1, y: 0 },
        "R" => Point { x: 1, y: 0 },
        _ => { panic!("Invalid direction {}", direction) }
    }
}

fn main() {
    let mut tail_visited = HashSet::<Point>::new();

    let mut rope = [Point::zero(); 10];

    tail_visited.insert(rope[1].clone());

    for line in lines_in_file("src/inputs/09.input") {
        let line = line.unwrap();
        let (direction, steps) = line.split_once(" ").unwrap();
        println!("Going {}, {}. Head is at {:?}, tail {:?}", direction, steps, rope[0], rope[1]);
        let increment = get_increment(direction);
        for _ in 0..steps.parse::<i32>().unwrap() {
            rope[0] = rope[0].plus(&increment);
            println!("          head is now at {:?}", rope[0]);
            for i in 1..rope.len() {
                if rope[i].is_far_from(&rope[i-1]) {
                    rope[i] = rope[i-1].get_closest_touching(&rope[i])
                }
            }
            tail_visited.insert(rope.last().unwrap().clone());
        }
    }

    println!("Total visited {:?}", tail_visited.len())
}
