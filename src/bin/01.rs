#[derive(Debug)]
#[derive(Clone)]
struct Elf {
    items: Vec<u32>,
    total: u32,
}

fn create_elf(items: Vec<u32>) -> Elf {
    let mut total = 0;
    for item in &items {
        total += item
    }
    Elf { items, total }
}

trait MaxTotal {
    fn three_max_totals(&self) -> (u32, u32, u32);
}

impl MaxTotal for Vec<Elf> {
    fn three_max_totals(&self) -> (u32, u32, u32) {
        let mut sorted = self.to_vec();
        sorted.sort_by(|elf1, elf2| elf2.total.cmp(&elf1.total));
        (sorted[0].total, sorted[1].total, sorted[2].total)
    }
}

use aoc2022::file_io::lines_in_file;

fn main(){
    let mut elves: Vec<Elf> = vec![];

    let mut items: Vec<u32> = vec![];

    for line in lines_in_file("src/inputs/01.input") {
        let string = line.unwrap();
        if string != "" {
            items.push(string.parse::<u32>().unwrap());
        } else {
            let elf = create_elf(items.clone());
            elves.push(elf);
            items.clear();
        };
    }
    elves.push(create_elf(items));

    for elf in &elves {
        println!("{:?}", elf);
    }

    let totals = elves.three_max_totals();
    println!("sum of ({:?}) = {}", totals, totals.0 + totals.1 + totals.2);

}
