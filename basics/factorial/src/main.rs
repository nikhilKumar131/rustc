fn main(){
    let num = 4;
    let mut factorial = 1;
    let mut factor = 1;
    loop{

        if factor == num {
            println!{"factorial of {} is {}", num, factorial};
            break;
        }
        factor +=1;
        factorial = factorial*factor;

    }
}