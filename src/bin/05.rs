use aoc2022::file_io::lines_in_file;
use regex::Regex;

#[derive(Debug)]
struct Instruction {
    count: u32,
    from: u32,
    to: u32,
}

impl Instruction {
    fn from(line: &str) -> Instruction {
        let line = line.replace("move ", "");
        let regex = Regex::new(r" from | to ").unwrap();
        let parts: Vec<String> = regex.split(&line).map(|x| x.to_string()).collect();
        Instruction {
            count: parts[0].parse::<u32>().unwrap(),
            from: parts[1].parse::<u32>().unwrap(),
            to: parts[2].parse::<u32>().unwrap(),
        }
    }

    fn perform_one_by_one(&self, stacks: &mut Vec<Vec<char>>) -> () {
        for _ in 1..self.count + 1 {
            let source = &mut stacks[self.from as usize];
            if let Some(x) = source.pop() {
                let target = &mut stacks[self.to as usize];
                target.push(x)
            } else {
                panic!()
            }
        }
    }

    fn perform_all_at_once(&self, stacks: &mut Vec<Vec<char>>) -> () {
        let source = &mut stacks[self.from as usize];
        let mut last = source.split_off(source.len() - self.count as usize);
        let target = &mut stacks[self.to as usize];
        target.append(&mut last)
    } 
}

fn build_stack(line: &str) -> Vec<char> {
    let stack = Vec::from_iter(line[1..].chars());
    stack
}

fn main() {
    let mut stacks: Vec<Vec<char>> = vec![Vec::new()];

    let mut is_building = true;
    for line in lines_in_file("src/inputs/05.input") {
        let line = line.unwrap();
        if is_building {
            // building stacks
            if line == "" {
                is_building = false;
                println!("{:?}", stacks);
                continue;
            }
            stacks.push(build_stack(&line))
        } else {
            let instruction = Instruction::from(&line);
            println!("{:?}", instruction);
            instruction.perform_all_at_once(&mut stacks);
            println!("{:?}", stacks)
        }
    }

    println!("{:?}", stacks);
    let last = stacks
        .iter()
        .map(|s| s.last())
        .map(|last| if let Some(x) = last { x } else { &' ' })
        .collect::<String>();
    println!("top crates: {:?}", last);
}
