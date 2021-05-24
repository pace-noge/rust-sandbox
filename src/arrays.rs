// Fixed list where elements are the same data type

use std::mem;


pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Re-assign value

    numbers[2] = 30;

    println!("{:?}", numbers);


    // get single
    println!("Single value: {}", numbers[0]);

    // get length
    println!("Array length {}", numbers.len());

    // Array are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));


    // Get slice

    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);
}