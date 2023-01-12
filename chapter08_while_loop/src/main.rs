use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut i = 0;
    let max = 5;

    while i < max {
        println!("Value: {i}");
        i += 1;
    }

    let mut j = 0;
    let maximum = 5;

    while j < maximum {
        let mut k = 0;
        let max_inner = j;

        while k <= max_inner {
            print!("* ");
            k += 1;
        }

        println!();
        j += 1;
    }

    let mut l = 0;
    let max_number = 5;

    while l < max_number {
        println!("Value: {i}");
        l += 1;

        sleep(Duration::from_secs(1));
    }
}
