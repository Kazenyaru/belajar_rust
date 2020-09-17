pub fn run() {
    println!("Hello World");

    // ? Basic Formating
    println!("Hello {}", "Fadil");
    println!("Hello {} from {}", "Fadil", "Rust");

    // ? Positional Arguments
    println!("Hello I'm {0} and {0} loves to {1}", "Fadil", "Code");

    // ? Named Arguments
    println!("{name} is learning {lang}", name = "Fadil", lang = "Rust");

    // ? Placeholder Traits
    println!("Binary: {:b}\nHex: {:x}\nOctal: {:o}", 10, 10, 10);

    // ? Debug Traits
    println!("{1:?} {0:?}", (1, 2, false, "Fadil"), (3, 4, true, "Fadil"));
}
