pub fn run() {
    let age = 18;

    if age >= 18 {
        println!("You'are mature enough");
    } else if age <= 18 {
        println!("Age little longer!");
    }

    let is_of_age = if age >= 18 { true } else { false };
    println!("is of age {}", is_of_age);
}
