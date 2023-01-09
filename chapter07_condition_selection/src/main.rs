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

    let numb_c = 10;
    if numb_c > 5 {
        println!("Congrat's, you pass");

        if numb_c == 10 {
            println!("With number is perfect");
        } else if numb_c > 7 {
            println!("With number is good");
        } else {
            println!("With number is enough");
        }
    } else {
        println!("You are not pass");

        if numb_c < 4 {
            println!("You can to try again");
        } else {
            println!("Don't forget to study and work hard");
        }
    }

    let numb_d = 3;
    let result_d: bool;

    if numb_d == 2 {
        result_d = false;
    } else {
        result_d = true;
    }

    println!("Result d is {result_d}");

    // is equivalen with result d

    let numb_e = 3;
    let result_e = if numb_e == 2 { true } else { false };

    println!("Result e is {result_e}");
}
