use rand::prelude::*;

fn main() {
    let q = 13;
    const n: usize = 8;
    let matrix: [[i32; n]; n] = genMatrix(q);

    let rng = rand::thread_rng();
    println!("S:");
    let sVector: [i32; n] = genSecret(q);
    println!("{:?}", sVector);

    let errorNoise: [i32; n] = genNoise(q);
    println!("E: ");
    println!("{:?}", errorNoise);

    let tVector = gentVector(q, sVector, matrix);
    println!("{:?}", tVector);
}

fn genNoise<const n: usize>(q: i32) -> [i32; n] {
    let mut errorNoise = [0i32; n];
    let mut rng = rand::thread_rng();
    for i in 0..n {
        errorNoise[i] = rng.gen_range(-1..2);
    }
    errorNoise
}

fn genSecret<const n: usize>(q: i32) -> [i32; n] {
    let mut sVector = [0i32; n];
    let mut rng = rand::thread_rng();
    for i in 0..n {
        sVector[i] = rng.gen_range(-1..2);
    }
    sVector
}
fn gentVector<const n: usize>(q: i32, sVector: [i32; n], matrix: [[i32; n]; n]) -> [i32; n] {
    let mut tVector = [0i32; n];
    for i in 0..n {
        let mut sum = 0;
        for j in 0..n {
            sum += sVector[j] * matrix[i][j];
        }
        tVector[i] = (sum % q).abs();
    }
    tVector
}

fn genMatrix<const n: usize>(q: i32) -> [[i32; n]; n] {
    let mut rng = rand::thread_rng();
    let mut matrix = [[0i32; n]; n];
    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = rng.gen_range(0..q - 1);
            let k = matrix[i][j];
            print!("{k} ");
        }
        print!("\n");
    }
    matrix
}
