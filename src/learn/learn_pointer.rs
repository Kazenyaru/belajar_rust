pub fn run() {
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("arr {:?} {:?}", arr1, arr2);

    let vecs1 = vec![4, 5, 6];
    let vecs2 = &vecs1;
    println!("vec {:?} {:?}", vecs1, vecs2);
}
