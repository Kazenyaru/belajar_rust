pub fn run() {
    let mut i: i32 = 0;
    println!("{0} and {0}", i);
    loop {
        i += 1;

        if i == 5 {
            continue;
        } else if i >= 10 {
            break;
        }

        println!("value i now {}", i);
    }

    let mut n = 1;
    while n <= 30 {
        n += 1;
        println!("{0}", n);
    }
}
