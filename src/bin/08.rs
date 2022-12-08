use aoc2022::file_io::lines_in_file;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn plus(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn is_within(&self, max_x: usize, max_y: usize) -> bool {
        self.x >= 0 && self.x < max_x as i32 && self.y >= 0 && self.y < max_y as i32
    }
}

type Trees = Vec<Vec<u8>>;

trait GetDimensions {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn size_at_point(&self, point: &Point) -> u8;
}

impl GetDimensions for Trees {
    fn get_width(&self) -> usize {
        self[0].len()
    }

    fn get_height(&self) -> usize {
        self.len()
    }

    fn size_at_point(&self, point: &Point) -> u8 {
        let x = point.x as usize;
        let y = point.y as usize;
        let size = self[y][x];
        size
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_increment(direction: &Direction) -> Point {
    use Direction::*;
    match direction {
        Up => Point { x: 0, y: -1 },
        Down => Point { x: 0, y: 1 },
        Left => Point { x: -1, y: 0 },
        Right => Point { x: 1, y: 0 },
    }
}

fn is_visible(tree: &Point, trees: &Trees, direction: &Direction) -> bool {

    let increment = get_increment(direction);

    let mut current_position = tree.clone();
    let w = trees.get_width();
    let h = trees.get_height();

    let original_tree_size = trees.size_at_point(&current_position);
    loop {
        current_position = current_position.plus(&increment);
        if !current_position.is_within(w, h) {
            break;
        }
        let current_tree_size = trees.size_at_point(&current_position);
        if current_tree_size >= original_tree_size {
            return false;
        }
    }
    true
}

fn get_score(tree: &Point, trees: &Trees, direction: &Direction) -> i32 {
    let increment = get_increment(direction);
    let mut current_position = tree.clone();
    let w = trees.get_width();
    let h = trees.get_height();

    let mut length = 0;
    let original_tree_size = trees.size_at_point(&current_position);
    loop {
        current_position = current_position.plus(&increment);
        if !current_position.is_within(w, h) {
            break;
        }
        length += 1;
        let current_tree_size = trees.size_at_point(&current_position);
        if current_tree_size >= original_tree_size {
            break;
        }
    }
    return length;
}

fn main() {
    let mut trees: Trees = vec![];
    for line in lines_in_file("src/inputs/08.input") {
        let line = line.unwrap();
        trees.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }

    let mut max_score = 0;
    let mut total_visible = 0;
    for y in 0..trees.get_height() {
        println!("==== checking row {} ====", y);
        for x in 0..trees.get_width() {
            let current_point = Point {
                x: x as i32,
                y: y as i32,
            };
            println!(
                "--- checking tree ({},{}): {} ---",
                x,
                y,
                trees.size_at_point(&current_point)
            );

            let visible_from_top = is_visible(&current_point, &trees, &Direction::Up);
            let visible_from_bottom = is_visible(&current_point, &trees, &Direction::Down);
            let visible_from_left = is_visible(&current_point, &trees, &Direction::Left);
            let visible_from_right = is_visible(&current_point, &trees, &Direction::Right);

            if visible_from_top {
                println!("    visible from Top")
            };
            if visible_from_bottom {
                println!("    visible from Bottom")
            };
            if visible_from_left {
                println!("    visible from Left")
            };
            if visible_from_right {
                println!("    visible from Right")
            };

            let visible =
                visible_from_top || visible_from_bottom || visible_from_left || visible_from_right;
            if visible {
                total_visible += 1;
            }

            println!("--- visible: {} ---\n", visible);

            let score_from_top = get_score(&current_point, &trees, &Direction::Up);
            let score_from_bottom = get_score(&current_point, &trees, &Direction::Down);
            let score_from_left = get_score(&current_point, &trees, &Direction::Left);
            let score_from_right = get_score(&current_point, &trees, &Direction::Right);

            let score = score_from_top * score_from_bottom * score_from_left * score_from_right;
            println!("--- scenic score: {} ---\n", score);
            if score > max_score {
                max_score = score;
            }
        }
    }

    //println!("{:#?}", visible_trees);
    println!("Total visible: {}", total_visible);
    println!("Max scenic score: {}", max_score);
}
