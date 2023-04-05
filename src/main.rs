mod keyGen;
mod reader;
mod writer;
fn main() {
    key_gen();
    reader::readPubKey("pub_256.mtx");
}

fn key_gen() {
    let q = 13;
    const N: usize = 256;
    let matrix: [[i32; N]; N] = keyGen::genMatrix(q);
    let sVector: [i32; N] = keyGen::genSecret(q);
    let errorNoise: [i32; N] = keyGen::genNoise(q);
    let tVector = keyGen::modMultiply(q, sVector, matrix);
    let tVector = keyGen::addVectors(errorNoise, tVector);
    writer::write_public_key(matrix, tVector);
    writer::write_private_key(sVector);
}
