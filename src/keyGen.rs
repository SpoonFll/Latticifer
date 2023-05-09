use rand::prelude::*;
/**
 * generates a matrix of noise to add errors to the public key and make the key harder to break
 */
pub fn genNoise<const N: usize, const K:usize>() -> [[i32; N];K] {
    let mut errorNoise = [[0i32; N];K];
    let mut rng = rand::thread_rng();
    for i in 0..N {
        for j in 0..K
        {
            errorNoise[j][i] = rng.gen_range(-1..2);
        }
    }
    errorNoise
}
/**
 * generates private key which is the matrix generated for the variables of the matrix
 * to be a solution for now it is only 1 wide however will soon be implemented to be k wide
 */
pub fn genSecret<const N: usize,const K: usize>(q: i32) -> [[i32; N];K] {
    let mut sVector = [[0i32; N];K];
    let mut rng = rand::thread_rng();
    for i in 0..N {
        for j in 0..K
        {
            sVector[j][i] = rng.gen_range(0..q);
        }
    }
    sVector
}
/**
 * multiplies two matricies however only one is N wide will soon have both be wider than 1 so we
 * can have N*N multiplied by N*K after multiplication the final vector has mod q applied to it to
 * become the answer
 */
pub fn modMultiply<const N:usize, const K:usize, const J:usize>(
    q: i32,
    vector_one: [[i32; N];J],
    vector_two: [[i32; N]; K],
) -> [[i32; J];K] {
    let mut vector_result = [[0i32; J];K];
    for i in 0..J
    {
        for j in 0..K
        {
            let mut sum = 0;
            for k in 0..N
            {
                sum += vector_one[i][k] * vector_two[j][k];
            }
            vector_result[j][i] = (sum % q).abs();
        }
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
        }
    }
    matrix
}
/**
 * add the elements of vectors together vec1[i] + vec2[i]
 */
pub fn addVectors<const N: usize,const K:usize>(vec_one: [[i32; N];K], vec_two: [[i32; N];K],q:i32) -> [[i32; N];K] {
    let mut result_vec = [[0i32; N];K];
    for i in 0..N {
        for j in 0..K
        {
            result_vec[j][i] = vec_one[j][i] + vec_two[j][i];
            result_vec[j][i] = (result_vec[j][i] % q).abs();
        }
    }
    result_vec
}

pub fn subVectors<const N: usize,const K:usize>(vec_one: [[i32; N];K], vec_two: [[i32; N];K],q:i32) -> [[i32; N];K] {
    let mut result_vec = [[0i32; N];K];
    for i in 0..N {
        for j in 0..K
        {
            result_vec[j][i] = vec_one[j][i] - vec_two[j][i];
            result_vec[j][i] = (result_vec[j][i] % q).abs();
        }
    }
    result_vec
}
