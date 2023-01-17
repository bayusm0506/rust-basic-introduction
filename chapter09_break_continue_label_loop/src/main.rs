fn main() {
    let mut i = 0;
    let max = 5;

    loop{
        println!("Value : {i}");
        i += 1;
        if i > max {
            break;
        }
    }
}
