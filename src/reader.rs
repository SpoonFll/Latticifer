use std::fs;
use std::fs::File;
use std::path::Path;
pub fn readKey(fileName: &str) -> Vec<Vec<i32>> {
    let path = Path::new(fileName);
    let display = path.display();
    let key = fs::read_to_string(fileName).expect("wrong");
    let lines: Vec<&str> = key.split("\n").collect();
    let mut matrix: Vec<Vec<i32>> = vec![];
    let mut lineNum = 0;
    for line in lines {
        let numbers: Vec<&str> = line.split(" ").collect();
        matrix.push(vec![]);
        for number in numbers.iter().skip(1) {
            matrix[lineNum].push(number.parse().unwrap());
        }
        lineNum += 1;
    }
    matrix
}
