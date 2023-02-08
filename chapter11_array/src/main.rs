
fn main() {
    let mut numbers = [24, 17, 32, 12];
    println!("Array {:?}", numbers);

    let data0 = numbers[0];
    println!("Array element of 0 {data0}");

    let data1 = numbers[1];
    println!("Array element of 1 {data1}");

    numbers[1] = 16;
    numbers[3] = 8;
    println!("Array {numbers:?}");

    let integer_numbers = [24, 17, 32, 12];
    println!("{integer_numbers:?}");

    let float_numbers = [24.1, 17.2, 32.3, 12.4];
    println!("{float_numbers:?}");

    let boolean_data = [false, true];
    println!("{boolean_data:?}");

    let integer_unsigned = [24, 21, 3];
    println!("{integer_unsigned:?}");

    let names = ["Jhon", "Rambo", "James"];
    let length = names.len();
    println!("array size is {}", length);

    let names2 = ["Vio", "Duck", "Cing"];
    for name in names2{
        println!("{name}")
    }

    let names3 = ["Chick", "Ant", "Mon"];
    for i in 0..names3.len(){
        println!("Array index ke-{i}: {}", names3[i]);
    }

    let data_arr = [
        ["salad", "fried rice"],
        ["apple", "coconut"],
        ["spinach", "jalapeno"]
    ];

    for sub_arr in data_arr {
        for el in sub_arr {
            print!("{el}, ");
        }
        println!();
    }

}
