use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
/**
 * writes matrix and t vector in a square in a text file in format m1 m2 m3 .... mN T1
 *                                                                 m1 m2 m3 .... mN T2
 * and ends with q at the end
 */
pub fn write_public_key<const N: usize, const K:usize>(matrix: [[i32; N]; N], t_vector: [[i32; N];K], q: i32) {
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
        for k in 0..K
        {
            to_string = format!("{} {}\n", to_string, t_vector[k][i]);
        }
    }
    to_string = format!("{} {}", to_string, q);
    match file.write_all(to_string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("wrote to {}", display),
    };
}
/**
 * writes the s vector and prime number q on one line in a text file
 */
pub fn write_private_key<const N: usize, const K:usize>(secret_vector: [[i32; N];K], q: i32) {
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
        for j in 0..K{
            to_string = format!("{} {}", to_string, secret_vector[j][i]);
        }
    }
    to_string = format!("{}\n {}", to_string, q);

    match file.write_all(to_string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("wrote to {}", display),
    };
}

pub fn write_private_message<const N:usize, const K:usize>(message_vector:[[i32;N];K])
{
    let mut fileName = String::new();
    fileName = format!("newMessage.perm");
    let path = Path::new(&fileName);
    let display = path.display();
    let mut file = match File::create(&path){
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file)=> file,
    };
    let mut to_string = String::new();
    for i in 0..N{
        for j in 0..K
        {
            to_string = format!("{} {}", to_string,message_vector[j][i]);
        }
    }
    match file.write_all(to_string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("wrote to {}", display),
    };

}
