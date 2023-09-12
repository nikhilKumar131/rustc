fn main() {
    let val: i32 = 10;
    let mut length = 1;
    let mut series: Vec<i32> = Vec::new();
    let mut last_value = 0;
    let mut second_last = 1;

    series.push(last_value);
    series.push(second_last);

    while length <= (val - 2) {

        let temp_second_last = last_value + second_last;
        last_value = second_last;
        second_last = temp_second_last;

        series.push(second_last);
        length += 1;
    }
    

    print!("{:?}",series);
}
