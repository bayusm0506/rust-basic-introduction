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

    let mut j = 0;
    let max_j = 5;

    loop{
        let mut k = max_j;
        let max_inner = j;

        loop {
            print!("* ");
            k -= 1;
            if k < max_inner{
                break;
            }
        }

        println!();

        j += 1;
        if j > max_j {
            break;
        }
    }

    let mut l = 0;
    let max_l = 15;

    loop {
        l += 1;

        if l % 2 == 1 {
            continue;
        }

        println!("Value i : {i}");

        if l > max_l {
            break;
        }
    }

    let mut m = 0;
    let max_m = 9;

    'mainloop: loop {
        m += 1;
        let mut n = 0;

        loop {
            if m > max_m {
                break 'mainloop;
            }

            n += 1;
            if n > m {
                break;
            }

            print!("{m} ");
        }

        println!();
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result : {result}")
}
