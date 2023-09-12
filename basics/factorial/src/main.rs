fn main(){
    let num = 4;
    let mut counter = 1;
    let mut factorial = 1;

    while counter <= num{
        factorial = factorial*counter;
        counter += 1;
    }

    print!{"{}", factorial};
}