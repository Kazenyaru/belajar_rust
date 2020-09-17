pub fn run() {
    hello_world(&5);
    let status = is_fadil("Fadil");
    println!("{}", status);

    // ? Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("C Sum: {}", add_nums(7, 8));
}

fn hello_world(n: &i32) {
    for i in 1..*n {
        println!("Hello world on index {}", i);
    }
    println!("{}", n);
}

fn is_fadil(value: &str) -> bool {
    println!("{}", value);
    if value == "Fadil" {
        return true;
    }
    return false;
}
