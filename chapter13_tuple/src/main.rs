fn main() {
    let tuple_a = ("Jhon", 27, ["racing", "working out"], true);
    println!("tuple_a: {:?}", tuple_a);

    println!("index 0: {:?}", tuple_a.0);
    println!("index 1: {:?}", tuple_a.1);
    println!("index 2: {:?} {:?}", tuple_a.2[0], tuple_a.2[1]);
    println!("index 3: {:?}", tuple_a.3);

    let mut tuple_b: (&str, i32, [&str; 2], bool) = ("default", 0, [""; 2], false);
    tuple_b.0 = "B";
    tuple_b.1 = 2;
    tuple_b.2 = ["Gaming", "Adventuring"];
    tuple_b.3 = true;

    println!("tuple_b: {:?}", tuple_b);
}
