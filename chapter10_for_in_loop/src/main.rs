fn main() {
    for i in 0..5 {
        println!("{i}");
    }

    'perulangan: for j in 0..=5 {
        if j > 3 {
            println!("stop loop to iteration {j}");
            break 'perulangan;
        }
    }
}
