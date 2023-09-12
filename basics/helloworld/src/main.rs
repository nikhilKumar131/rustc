// fn main() {
//     // println!("Hello, world!");
//     let name = "Nikhil";
//     let age = 22;

//     println!("{}'s age is {}", name, age);
// }


/*
fn add(a: i32, b: i32) -> i32 {
    let c = a + b;
    return c;
}

fn main(){
    let a: i32 = 12;
    let b: i32 = 13;
    let sum: i32 = add(a,b);

    println!("sum of {} and {} is {}",a,b,sum);
}
*/

// fn main(){
//     let range: [i32]   = [1, "two", "threee", "four"];
//     for i in range{
//         println!("{}",i);
//     }
// }

fn main(){
    let mut a = 10;

    println!("value directly i32 is {}", a);

    let  mut b = &a;
    println!("value of &a is {}", b);
    let binding = b + 5;
    b = &binding;
    println!("value of &a is {}", b);


    let c = &mut a;
    println!("value of &mut a is {}", c);

}