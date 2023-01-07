fn main() {
    let (num1, num2): (i32, i32) = (12, 4);

    let val_add: i32 = num1 + num2;
    println!("{} + {} = {}", num1, num2, val_add);

    let val_sub: i32 = num1 - num2;
    println!("{} - {} = {}", num1, num2, val_sub);

    let val_mut: i32 = num1 * num2;
    println!("{} * {} = {}", num1, num2, val_mut);

    let val_div: i32 = num1 / num2;
    println!("{} / {} = {}", num1, num2, val_div);

    let val_mod: i32 = num1 % num2;
    println!("{} % {} = {}", num1, num2, val_mod);

    let numb_a: i32 = 12;
    let numb_b: i32 = 24;

    let res_eq: bool = numb_a == numb_b;
    println!("{} == {} = {}", numb_a, numb_b, res_eq);

    let res_not_eq: bool = numb_a != numb_b;
    println!("{} != {} = {}", numb_a, numb_b, res_not_eq);

    let res_big: bool = numb_a > numb_b;
    println!("{} > {} = {}", numb_a, numb_b, res_big);

    let res_small: bool = numb_a < numb_b;
    println!("{} < {} = {}", numb_a, numb_b, res_small);

    let res_big_than: bool = numb_a >= numb_b;
    println!("{} >= {} = {}", numb_a, numb_b, res_big_than);

    let res_small_than: bool = numb_a <= numb_b;
    println!("{} <= {} = {}", numb_a, numb_b, res_small_than);

    println!("res_eq : {res_eq}");
    println!("res_not_eq : {res_not_eq}");
}
