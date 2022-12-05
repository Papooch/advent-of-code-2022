
use aoc2022::file_io::lines_in_file;

#[derive(Debug)]
#[derive(Clone)]
struct Range {
    low: u32,
    high: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    fn disjoint(&self, other: &Range) -> bool {
        self.high < other.low || self.low > other.high
    }
}

fn get_range(range_str: String) -> Range {
    let both = range_str.split_terminator("-").collect::<Vec<&str>>();
    let low = both[0].parse::<u32>().unwrap();
    let high = both[1].parse::<u32>().unwrap();
    Range { low, high }
}

fn get_ranges(line: String) -> (Range, Range) {
    let both = line.split_terminator(",").collect::<Vec<&str>>();
    let left = both[0].to_owned();
    let right = both[1].to_owned();
    (get_range(left), get_range(right)).to_owned()
}

fn main() {
    let mut total_containing = 0;
    let mut total_overlapping = 0;

    for line in lines_in_file("src/inputs/04.input") {
        let line = line.unwrap();
        let (range1, range2) = get_ranges(line);
        println!("left: {:?}, right: {:?}", range1, range2);
        if range1.contains(&range2) || range2.contains(&range1) {
            println!("contains");
            total_containing += 1
        }
        if !range1.disjoint(&range2) {
            println!("overlaps");
            total_overlapping += 1
        }
    }

    println!("total containing: {}, total overlapping: {}", total_containing, total_overlapping)
}