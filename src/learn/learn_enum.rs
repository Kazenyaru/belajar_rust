pub fn run() {
    enum Direction {
        Left,
        Up,
        Down,
        Right,
    }
    let movement: Direction = Direction::Up;
    match movement {
        Direction::Left => println!("Heading Up"),
        Direction::Up => println!("Heading Up"),
        Direction::Down => println!("Heading Up"),
        Direction::Right => println!("Heading Up"),
    }

    const MAC_NUM: u32 = 20;
    println!("Here is the maximum number {0}", MAC_NUM);
}
