pub fn run() {
    let name = "Fadil";
    let mut age = 16;

    age += 1;
    println!("My name is {} and I am {}", name, age);

    // ? Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // ? Assign
    let (my_name, my_age) = (name, age);
    println!("I am {}, I am {}", my_name, my_age);
}
