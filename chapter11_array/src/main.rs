
fn main() {
    let mut numbers = [24, 17, 32, 12];
    println!("Array {:?}", numbers);

    let data0 = numbers[0];
    println!("Array element of 0 {data0}");

    let data1 = numbers[1];
    println!("Array element of 1 {data1}");

    numbers[1] = 16;
    numbers[3] = 8;
    println!("Array {numbers:?}")
}
