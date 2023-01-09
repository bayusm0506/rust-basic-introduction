fn main() {
    let numb_a = 3;
    if numb_a < 5 {
        println!("Number a is left than 5");
    }

    let result_a = numb_a >= 5;
    if result_a {
        println!("Result a is more than or equals 5");
    }

    let numb_b = 3;
    if numb_b == 2 {
        println!("Number b is 2");
    } else if numb_b < 2 {
        println!("Number b is left than 2");
    } else {
        println!("Number b is more than 2");
    }
}
