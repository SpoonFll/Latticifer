mod keyGen;
mod reader;
mod writer;
fn main() {
    key_gen();

    let matrixPub: Vec<Vec<i32>> = reader::readKey("pub_256.mtx");
    let matrixPriv: Vec<Vec<i32>> = reader::readKey("priv_256.mtx");
    //println!("{:?}", matrixPriv);
    //println!("{:?}", matrixPub);
    encode();
    decode();
}

/**
 * keygen randomly generates matricies with randomly generated numbers based on size N and a prime
 * number q to do the operations to generate both the public and private keys
 */
fn key_gen() {
    let q = 13;
    const N: usize = 256;
    const K: usize = 1;
    let matrix: [[i32; N]; N] = keyGen::genMatrix(q);
    let sVector: [[i32; N];K] = keyGen::genSecret(q);
    let errorNoise: [[i32; N];K] = keyGen::genNoise();
    let tVector: [[i32;N];K] = keyGen::modMultiply(q, matrix, sVector);
    let tVector = keyGen::addVectors(errorNoise, tVector,q);
    writer::write_public_key(matrix, tVector, q);
    writer::write_private_key(sVector, q);
}

fn decode()
{
    const N: usize = 256;
    const K: usize = 1;
    let message: Vec<Vec<i32>> = reader::readKey("newMessage.perm"); 
    let matrixPriv: Vec<Vec<i32>> = reader::readKey("priv_256.mtx");
    let q = *matrixPriv.get(matrixPriv.len()-1).unwrap().get(0).unwrap();
    let mut messageMatrixLong = [[0i32;N];K];
    let mut secretKey = [[0i32;N];K];
    let mut messageMatrixShort = [[0i32;K];K];

    println!("{:?}",matrixPriv);
    for i in 0..N
    {
        for j in 0..K
        {
            messageMatrixLong[j][i] = *message.get(j).unwrap().get(i).unwrap();
            secretKey[j][i] = *matrixPriv.get(j).unwrap().get(i).unwrap();
        }
    }
    for i in 0..K 
    {
        for j in 0..K
        {
            messageMatrixShort[j][i] = *message.get(j).unwrap().get(i+N).unwrap();
        }
    }   

    let mut result:[[i32;K];K] = keyGen::modMultiply(q, messageMatrixLong, secretKey);
    result = keyGen::subVectors(messageMatrixShort, result,q);

    for i in 0..K 
    {
        for j in 0..K
        {
            if((result[j][i]>(q/4))&& (result[j][i]<(3*q/4)))
            {
                result[j][i]=1;
            }
            else
            {
                result[j][i]=0;
            }
        }
    }
    println!("{:?}",result);
}

fn encode()
{
    const N: usize = 256;
    const K: usize = 1;
   
    let public_data: Vec<Vec<i32>> = reader::readKey("pub_256.mtx"); 
    let q = *public_data.get(public_data.len()-1).unwrap().get(0).unwrap();
    //println!("{}",q);
    let mut mutliMatrix = [[0i32;N];N+K];
    for i in 0..N
    {
        for j in 0..N+K
        {
            mutliMatrix[j][i] = *public_data.get(i).unwrap().get(j).unwrap();
        }
    }
   
    let new_secret:[[i32;N];K] = keyGen::genSecret(q);
    let new_error:[[i32;N+K];K] = keyGen::genNoise();
     
    let message = 0;
    let mut new_product:[[i32;N+K];K] = keyGen::modMultiply(q, mutliMatrix,new_secret);
    new_product = keyGen::addVectors(new_product, new_error,q);
    new_product[K-1][N+K-1] +=message; 
    writer::write_private_message(new_product);
}
