use sha2::{Sha256, Digest};

fn main(){
    // input and initialze
    let input = "Hello, world!";
    let mut sha = Sha256::new();
    //update and finalize
    sha.update(input);
    let result = sha.finalize();

    //result encoding and printing
    let value = hex::encode(result);
    println!("hased value of {} is {}",input, value);

}