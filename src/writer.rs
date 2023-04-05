use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn write_public_key<const N: usize>(matrix: [[i32; N]; N], t_vector: [i32; N]) {
    let mut fileName = String::new();
    fileName = format!("pub_{N}.mtx");
    let path = Path::new(&fileName);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    let mut to_string = String::new();
    for i in 0..N {
        for j in 0..N {
            to_string = format!("{} {}", to_string, matrix[i][j]);
        }
        to_string = format!("{} {}\n", to_string, t_vector[i]);
    }
    match file.write_all(to_string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("wrote to {}", display),
    };
}
pub fn write_private_key<const N: usize>(secret_vector: [i32; N]) {
    let mut fileName = String::new();
    fileName = format!("priv_{N}.mtx");
    let path = Path::new(&fileName);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut to_string = String::new();

    for i in 0..N {
        to_string = format!("{} {}", to_string, secret_vector[i]);
    }

    match file.write_all(to_string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("wrote to {}", display),
    };
}
