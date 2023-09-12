// use rand::Rng;

// fn create_private_key() -> [u8;32]{
//     let mut rng = rand::thread_rng();
//     let private_key: [u8; 32] = rng.gen();
//     private_key
// }

// fn main(){
//     //calls create_private_key
//     let key: [u8;32] = create_private_key()
//     //prints the key
//     println!("the new private key is {:?}", key)
// }


use rand::Rng;

fn generate_private_key() -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let private_key: [u8; 32] = rng.gen();
    private_key
}

fn main() {
    let private_key = generate_private_key();
    println!("Private Key: {:?}", private_key);
}
