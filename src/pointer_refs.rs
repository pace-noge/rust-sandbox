pub fn run() {
    // primitive array
    let arr1 = [1, 2,3];
    let arr2 = arr1;

    println!("{:?}", (arr1, arr2));

    // non primitive we need to use (&) to point to resource

    let vec1 = vec![1, 2, 3, 4];
    let vec2 = &vec1;

    println!("{:?}", (&vec1, vec2));
}