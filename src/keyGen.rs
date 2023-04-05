use rand::prelude::*;
pub fn genNoise<const N: usize>(q: i32) -> [i32; N] {
    let mut errorNoise = [0i32; N];
    let mut rng = rand::thread_rng();
    for i in 0..N {
        errorNoise[i] = rng.gen_range(-1..2);
    }
    errorNoise
}

pub fn genSecret<const N: usize>(q: i32) -> [i32; N] {
    let mut sVector = [0i32; N];
    let mut rng = rand::thread_rng();
    for i in 0..N {
        sVector[i] = rng.gen_range(-1..2);
    }
    sVector
}
pub fn modMultiply<const N: usize>(
    q: i32,
    vector_one: [i32; N],
    vector_two: [[i32; N]; N],
) -> [i32; N] {
    let mut vector_result = [0i32; N];
    for i in 0..N {
        let mut sum = 0;
        for j in 0..N {
            sum += vector_one[j] * vector_two[i][j];
        }
        vector_result[i] = (sum % q).abs();
    }
    vector_result
}
pub fn genMatrix<const N: usize>(q: i32) -> [[i32; N]; N] {
    let mut rng = rand::thread_rng();
    let mut matrix = [[0i32; N]; N];
    for i in 0..N {
        for j in 0..N {
            matrix[i][j] = rng.gen_range(0..q - 1);
            let k = matrix[i][j];
        }
    }
    matrix
}
pub fn addVectors<const N: usize>(vec_one: [i32; N], vec_two: [i32; N]) -> [i32; N] {
    let mut result_vec = [0i32; N];

    for i in 0..N {
        result_vec[i] = vec_one[i] + vec_two[i];
    }
    result_vec
}
