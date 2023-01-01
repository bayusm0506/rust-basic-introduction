fn main() {
    let numeric1 = 24;
    let numeric2: i8 = 127;
    let numeric3: i64 = 12;
    println!("{} | {} | {}", numeric1, numeric2, numeric3);

    let numeric4: u32 = 28;
    let numeric5: u8 = 16;
    let numeric6: u64 = 42;
    println!("{} | {} | {}", numeric4, numeric5, numeric6);

    let fp1: f32 = 3.14;
    let fp2: f64 = 3.1415926535;
    println!("{} | {:.5}", fp1, fp2);

    let b1 = false;
    let b2 = true;
    println!("{} | {}", b1, b2);

    let c1 = 'n';
    let c2 = '-';
    let c3 = '2';
    println!("{} | {} | {}", c1, c2, c3);

    let ptr1: &i32 = &24;
    println!("{}", ptr1);
}
