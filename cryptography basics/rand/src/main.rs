// use rand::Rng;

// fn main() {
//     let mut rng = rand::thread_rng();

//     // Generate a random integer between 1 and 100
//     let random_number = rng.gen_range(1..=100);
//     println!("Random Number: {}", random_number);

//     // Generate a random boolean
//     let random_bool: bool = rng.gen();
//     println!("Random Boolean: {}", random_bool);

//     // Generate a random floating-point number between 0 and 1
//     let random_float: f64 = rng.gen();
//     println!("Random Float: {}", random_float);
// }



use rand::Rng;

fn main(){
    let mut rng = rand::thread_rng();

    //generate and pring bool

    let bool_val: bool = rng.gen();
    println!("bool: {}", bool_val);

    let num_val: i32 = rng.gen();
    println!("int: {}", num_val);

    //print a random vector of 10 digits range 10:50 inclusive
    let arr: Vec<_> =(0..10).map(|_| rng.gen_range(10..=50)).collect();
    println!("array: {:?}" , arr);
}
