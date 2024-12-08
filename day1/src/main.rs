use std::fs;
use std::path::Path;

fn main() {
    let (column1, column2): (Vec<i32>, Vec<i32>) = read_input().unwrap();
}

fn read_input() -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>> {
    let path = Path::new("./test");
    let content = fs::read_to_string(path)?;

    let (column1, column2): (Vec<i32>, Vec<i32>) = content
        .lines()
        .filter_map(|line| {
            line.split_whitespace()
                .map(str::parse::<i32>)
                .collect::<Result<Vec<_>, _>>()
                .ok()
                .and_then(|nums| match nums.as_slice() {
                    [num1, num2] => Some((*num1, *num2)),
                    _ => None,
                })
        })
        .unzip();

    // println!("Column 1: {:?}", column1);
    // println!("Column 2: {:?}", column2);

    Ok((column1, column2))
}