mod keyGen;
mod reader;
mod writer;
fn main() {
    key_gen();

    let matrixPub: Vec<Vec<i32>> = reader::readKey("pub_256.mtx");
    let matrixPriv: Vec<Vec<i32>> = reader::readKey("priv_256.mtx");
    println!("{:?}", matrixPriv);
    println!("{:?}", matrixPub);
}

/**
 * keygen randomly generates matricies with randomly generated numbers based on size N and a prime
 * number q to do the operations to generate both the public and private keys
 */
fn key_gen() {
    let q = 13;
    const N: usize = 256;
    let matrix: [[i32; N]; N] = keyGen::genMatrix(q);
    let sVector: [i32; N] = keyGen::genSecret(q);
    let errorNoise: [i32; N] = keyGen::genNoise();
    let tVector = keyGen::modMultiply(q, sVector, matrix);
    let tVector = keyGen::addVectors(errorNoise, tVector);
    writer::write_public_key(matrix, tVector, q);
    writer::write_private_key(sVector, q);
}

fn decode()
{

}

fn encode()
{
   let q = 13;
   const N: usize = 256;
   const K: usize = 1;
   let public_data: Vec<Vec<i32>> = reader::readKey("pub_256.mtx"); 
   let mut mutliMatrix = [[0i32;N];N+K];
   for i in 0..N
   {
       for j in 0..N+K
       {
            mutliMatrix[i][j] = *public_data.get(i).unwrap().get(j).unwrap();

       }
   }
   let new_secret:[i32;N] = keyGen::genSecret(q);
   let new_error:[i32;N+K] = keyGen::genNoise();
   let message = 1;
   let new_product:[i32;N+K] = keyGen::modMultiply(q, mutliMatrix,new_secret);
   new_product = keyGen::addVectors(new_product, new_error);
   new_product[N+K] += message;
}
