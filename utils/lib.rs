use std::fs;

pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Unable to read input file")
}

pub fn read_input_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Unable to read input file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}
