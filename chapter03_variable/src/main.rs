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

    let (var5, mut var6, var7): (i8, i8, i8) = (64, 12, 4);
    println!("Var5 is {0}", var5);
    println!("Var6 is {0}", var6);
    println!("Var7 is {0}", var7);

    var6 = 24;
    println!("Var6 is {0}", var6);
    println!("Var7 is {0}", var7);

    let data1 = 24i8;
    println!("Data1 is {0}", data1);
    // or
    let data2 = 24_i8;
    println!("Data2 is {0}", data2);
    
}