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
    println!("Number is {1}: {0}", number, message3);

    let (var1, var2) = (24, "Hello");
    println!("Var1 is {0}", var1);
    println!("Var2 is {0}", var2);

    let (var3, var4): (i8, i8) = (24, 12);
    println!("Var3 is {0}", var3);
    println!("Var4 is {0}", var4);
}