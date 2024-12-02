use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn read_lines_in_file(filename: &str) -> Vec<String> {
    let mut vec = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            vec.push(line.trim().to_string());
        }
    }
    vec
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
