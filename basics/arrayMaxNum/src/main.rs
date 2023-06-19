fn find_max(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }

    let mut max = numbers[0];
    for &num in numbers.iter() {
        if num > max {
            max = num;
        }
    }

    Some(max)
}

fn main() {
    let numbers = [2, 8, 5, 10, 1];
    match find_max(&numbers) {
        Some(max) => println!("The maximum number is: {}", max),
        None => println!("No numbers found"),
    }
}
