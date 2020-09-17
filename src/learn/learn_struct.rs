use std::fmt;

// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: String::from(first),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", &self.first_name, &self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
}

pub fn run() {
    // let colours = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // print!(
    //     "the colours {} {} {}",
    //     colours.red, colours.green, colours.blue
    // );

    // let colours = Color(255, 0, 0);
    let mut p = Person::new("Eurus", "Holmes");
    println!("The name is: {}", p.first_name);
    println!("Fullname: {}", p.full_name());
    p.set_last_name("Watson");
    println!("The new name is: {}", p.full_name());
}
