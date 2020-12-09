use std::{fs::File, iter::FromIterator};
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_and_check_input<Collection>(filename: &str) -> Collection
where Collection: FromIterator<String> {
    if let Ok(lines) = read_lines(filename) {
        lines.map(|line| {
            if let Ok(line) = line {
                line
            } else {
                let panic_string = {
                    let mut panic_string = "Error when rading a line in file ".to_owned();
                    panic_string.push_str(filename);
                    panic_string
                };
                panic!(panic_string);
            }
        }).collect()
    } else {
        let panic_string = {
            let mut panic_string = "File".to_owned();
            panic_string.push_str(filename);
            panic_string.push_str(" not found!");
            panic_string
        };
        panic!(panic_string);
    }
}
