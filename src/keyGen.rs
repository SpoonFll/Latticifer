use rand::prelude::*;
/**
 * generates a matrix of noise to add errors to the public key and make the key harder to break
 */
pub fn genNoise<const N: usize>(q: i32) -> [i32; N] {
    let mut errorNoise = [0i32; N];
    let mut rng = rand::thread_rng();
    for i in 0..N {
        errorNoise[i] = rng.gen_range(-1..2);
    }
    errorNoise
}
/**
 * generates private key which is the matrix generated for the variables of the matrix
 * to be a solution for now it is only 1 wide however will soon be implemented to be k wide
 */
pub fn genSecret<const N: usize>(q: i32) -> [i32; N] {
    let mut sVector = [0i32; N];
    let mut rng = rand::thread_rng();
    for i in 0..N {
        sVector[i] = rng.gen_range(-1..2);
    }
    sVector
}
/**
 * multiplies two matricies however only one is N wide will soon have both be wider than 1 so we
 * can have N*N multiplied by N*K after multiplication the final vector has mod q applied to it to
 * become the answer
 */
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
/**
 * generates matrix based on a range of 0-(q-1) for mod q to work properly in the above function
 */
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
/**
 * add the elements of vectors together vec1[i] + vec2[i]
 */
pub fn addVectors<const N: usize>(vec_one: [i32; N], vec_two: [i32; N]) -> [i32; N] {
    let mut result_vec = [0i32; N];

    for i in 0..N {
        result_vec[i] = vec_one[i] + vec_two[i];
    }
    result_vec
}
