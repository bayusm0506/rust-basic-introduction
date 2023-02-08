fn main() {
    let numbers = [12, 16, 8, 3];
    println!("numbers   : {:?}, len: {}", numbers, numbers.len());
    println!("numbers[0] : {:?}", numbers[0]);
    println!("numbers[1] : {:?}", numbers[1]);

    let slice_a = &numbers[0..3];
    println!("slice_a : {:?}, len: {}", slice_a, slice_a.len());
    println!("slice_a[0] : {:?}", slice_a[0]);
    println!("slice_b[1] : {:?}", slice_a[1]);

    let slice_b = &slice_a[1..=2];
    println!("slice_b : {:?}, len: {}", slice_b, slice_b.len());
    println!("slice_b[0] : {:?}", slice_b[0]);
    println!("slice_b[1] : {:?}", slice_b[1]);
}
