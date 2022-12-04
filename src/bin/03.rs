use std::collections::HashSet;

use aoc2022::file_io::lines_in_file;

fn get_score(ch: char) -> u32 {
    let num = ch as u32;
    if num > 97 {
        return num - 96;
    } else {
        return num - 64 + 26;
    }
}

fn main() {
    let mut total = 0;

    let mut lines: Vec<String> = vec![];
    for line in lines_in_file("src/inputs/03.input") {
        let line = line.unwrap();
        if line == "" {
            break;
        }
        lines.push(line.clone());

        if lines.len() < 3 {
            continue;
        };

        let item_set1 = HashSet::<char>::from_iter(lines[0].chars());
        let item_set2 = HashSet::<char>::from_iter(lines[1].chars());
        let item_set3 = HashSet::<char>::from_iter(lines[2].chars());

        let common_items_in_first_two = item_set1.intersection(&item_set2);
        let mut common: Option<char> = None;
        for common_item in common_items_in_first_two {
            let common_item = common_item.to_owned();
            if item_set3.contains(&common_item) {
                common = Some(common_item)
            }
        }

        println!("{:?}", common);
        total += get_score(common.unwrap());

        lines.clear()
    }
    println!("total: {}", total)
}
