use std::fs;

fn main() {
    let file_path = "test.md";

    let content = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let c: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    for line in &c {
        println!("{:?}", line);
    }
}
