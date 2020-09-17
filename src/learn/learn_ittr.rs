pub fn run() {
    let tupl = (20, "Fadil", (1, 2, 3));
    let (_, b, _) = tupl;
    println!(
        "here is tupple nested {0} and destructured tupple {1}",
        (tupl.2).0,
        b
    );
    print!("{:?}", tupl);

    // ? Array
    let mut arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    // ? Slice Array
    let slice: &[i32] = &arr[0..2];
    println!("{:?}", slice);

    // ? Range
    for i in 1..11 {
        println!("number is {}", i);
    }

    // ? Vector
    let mut nums = vec![1, 2, 3, 4];
    for (index, n) in nums.iter().enumerate() {
        println!("Number: {0} is in Index: {1}", n, index);
    }

    nums.push(22);
    println!("{:?}", nums);
}
