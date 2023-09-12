fn main() {
    let _a = 10;
    let _b = 4;
    find_lcm(&_a,&_b);
}


fn find_lcm(_a: &i32, _b: &i32) {

    
    let a = if _a > _b { _b } else { _a };
    let mut cf: Vec<i32> = Vec::new(); // Create a vector to store factors
    let mut lcm = _a*_b;


    for i in 1..*a{
        if _a % i == 0 {
            cf.push(i);
            lcm = lcm/i
        }
    }



    println!("{:?}", cf);
    println!("LCM of {} and {} is {}",_a, _b, lcm);
}
