// use sha2::{Sha256, Digest};

// fn main(){
//     // input and initialze
//     let input = "Hello, world!";
//     let mut sha = Sha256::new();
//     //update and finalize
//     sha.update(input);
//     let result = sha.finalize();

//     //result encoding and printing
//     let value = hex::encode(result);
//     println!("hased value of {} is {}",input, value);

// }


//Code2
use sha2::{Sha256,Digest};

fn main(){
    let massege = "my world is beautiful";

    let hashed = hash_function(&massege);

    println!("messege: {}", massege);
    println!("hashed: {}", hashed);

}

fn hash_function(massege:&str) -> String{

    let mut sh = Sha256::new();
    sh.update(massege);

    let result = sh.finalize();
    let val = hex::encode(result);

    val

}