use std::f32::consts::E;

use aoc2022::file_io::lines_in_file;

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
struct Dir {
    id: usize,
    name: String,
    dir_ids: Vec<usize>,
    files: Vec<File>,
    parent_id: usize,
}

impl Dir {
    fn new(parent_id: usize, id: usize, name: &str) -> Dir {
        Dir {
            id,
            parent_id,
            name: name.to_owned(),
            dir_ids: Vec::new(),
            files: Vec::new(),
        }
    }

    fn get_size(&self, dirs: &Vec<Dir>) -> u32 {
        let mut total: u32 = 0;
        for dir_id in &self.dir_ids {
            total += dirs[*dir_id].get_size(dirs)
        }
        for file in &self.files {
            total += file.size
        }
        total
    }

    fn add_subfolder(&mut self, sub_id: usize) -> () {
        self.dir_ids.push(sub_id);
    }

    fn add_file(&mut self, name: &str, size: u32) {
        self.files.push(File {
            name: name.to_owned(),
            size,
        })
    }

    fn visit_dir(&self, name: &str, dirs: &Vec<Dir>) -> usize {
        let dir = self
            .dir_ids
            .iter()
            .map(|id| &dirs[*id])
            .find(|d| d.name == name);
        dir.unwrap().id.to_owned()
    }

    fn print(&self, dirs: &Vec<Dir>, indent: usize) {
        println!(
            "{:indent$}- {} (dir, size={})", "",
            self.name,
            self.get_size(dirs),
            indent=indent * 2
        );
        for dir_id in &self.dir_ids {
            dirs[*dir_id].print(dirs, indent + 1)
        }
        for file in &self.files {
            println!(
                "{:indent$}- {} (file, size={})", "",
                file.name,
                file.size,
                indent=indent * 2 + 1
            )
        }
    }
}

enum Command {
    Root,
    Up,
    Into(String),
    List,
}

fn is_command(line: &str) -> bool {
    line.chars().collect::<Vec<char>>()[0] == '$'
}

fn parse_command(line: &str) -> Command {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    println!("parsing command, {:?}", parts);
    match parts[1] {
        "cd" => match parts[2] {
            ".." => Command::Up,
            "/" => Command::Root,
            dir => Command::Into(dir.to_owned()),
        },
        "ls" => Command::List,
        x => panic!("Unknown command {}", x),
    }
}

fn main() {
    let root = Dir {
        id: 0,
        parent_id: 0,
        name: String::from("/"),
        dir_ids: Vec::new(),
        files: Vec::new(),
    };

    let mut dirs = vec![root];
    let mut current_id: usize = 0;

    let mut is_reading = false;
    // skip the first line, because it's always `cd /`
    for line in lines_in_file("src/inputs/07.input").skip(1) {
        let line = line.unwrap();

        if is_command(&line) {
            is_reading = false;
            match parse_command(&line) {
                Command::List => is_reading = true,
                Command::Root => current_id = 0,
                Command::Into(dir) => {
                    current_id = dirs[current_id].visit_dir(&dir, &dirs);
                }
                Command::Up => {
                    current_id = dirs[current_id].parent_id;
                }
            }
        } else if is_reading {
            println!("Reading dir entry '{}'", line);
            let (dir_or_size, name) = line.split_once(" ").unwrap();
            if dir_or_size == "dir" {
                let new_dir = Dir::new(current_id, dirs.len(), &name);
                dirs[current_id].add_subfolder(new_dir.id);
                dirs.push(new_dir);
            } else {
                dirs[current_id].add_file(name, dir_or_size.parse::<u32>().unwrap())
            }
            continue;
        } else {
            panic!(
                "We're in command mode, but the line {} is not a command!",
                line
            )
        }
    }

    dirs[0].print(&dirs, 0);

    let small_dirs_size = dirs.iter().filter(|dir| dir.get_size(&dirs) < 1_00_000).map(|dir| dir.get_size(&dirs)).sum::<u32>();
    println!("{}", small_dirs_size);

    let total_size = dirs[0].get_size(&dirs);
    println!("Total size: {}", total_size);
    println!("Total unused: {}", 70_000_000 - total_size);
    let extra_size_needed = 30_000_000 - (70_000_000 - total_size);
    println!("Needed to free: {}", extra_size_needed);

    let mut delete_candidate_id = 0;
    let mut delete_candidate_size = total_size;
    for dir in &dirs {
        let dir_size = dir.get_size(&dirs);
        if dir_size >= extra_size_needed && dir_size < delete_candidate_size {
            delete_candidate_id = dir.id;
            delete_candidate_size = dir_size;
        }
    }

    println!("Should delete {} with size: {}", delete_candidate_id, delete_candidate_size);
}
