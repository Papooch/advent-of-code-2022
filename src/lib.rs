pub mod file_io {
    use std::fs::File;
    use std::io::{Lines, prelude::*, BufReader};

    pub fn lines_in_file(path: &str) -> Lines<BufReader<File>> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        return reader.lines()
    }
}