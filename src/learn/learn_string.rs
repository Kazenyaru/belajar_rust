pub fn run() {
    let mut hello = String::from("Hello");
    println!("This is String {} the length is {}", hello, hello.len());

    hello.push_str(" Fadil \u{1F600}");
    println!("This is after push_str {}", hello);

    println!("Capacity: {}", hello.capacity());
    println!("Is Empty: {}", hello.is_empty());
    println!("Contain 'Hello' {}", hello.contains("Hello"));
    println!("Replace 'Hello' {}", hello.replace("Hello", "Holla"));

    println!("\n\nSplit Whitespace");
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    assert_eq!(2, hello.len());
}
