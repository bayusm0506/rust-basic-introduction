fn main() {
    let (num1, num2): (i32, i32) = (12, 4);

    let val_add:i32 = num1 + num2;
    println!("{} + {} = {}", num1, num2, val_add);

    let val_sub:i32 = num1 - num2;
    println!("{} - {} = {}", num1, num2, val_sub);

    let val_mut:i32 = num1 * num2;
    println!("{} * {} = {}", num1, num2, val_mut);

    let val_div:i32 = num1 / num2;
    println!("{} / {} = {}", num1, num2, val_div);

    let val_mod:i32 = num1 % num2;
    println!("{} % {} = {}", num1, num2, val_mod);
}
