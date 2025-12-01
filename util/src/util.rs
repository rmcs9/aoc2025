use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;



pub fn get_data(fname: &str) -> Vec<String> {
    let path: &str = &format!("{}", fname);
    let path = Path::new(path);

    let file = match File::open(path) {
        Err(why) => panic!("Failed to open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut lines: Vec<String> = vec![];
    let freader = BufReader::new(file);

    for line in freader.lines() {
        lines.push(line.unwrap());
    }

    return lines;
}
