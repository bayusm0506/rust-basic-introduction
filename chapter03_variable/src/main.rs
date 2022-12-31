fn main() {
    // {} : string literal

    let variable_name = "Predefine Value";
    println!("{}", variable_name);

    let mut number = 1;
    let message = "Hello";
    println!("Number is {}: {}", number, message);

    number = 2;
    let message2 = "World";
    println!("Number is {}: {}", number, message2);

    number = 3;
    let message3: i8 = 24;
    println!("Number is {1}: {0}", number, message3)
}