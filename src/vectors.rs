// Vectors: resizable arrays

use std::mem;


pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Re-assign value

    numbers[2] = 30;

    println!("{:?}", numbers);


    // get single
    println!("Single value: {}", numbers[0]);

    // get length
    println!("Vector length {}", numbers.len());

    // Array are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));


    // Get slice

    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);

    numbers.push(50);
    println!("Push 50 to numbers {:?}", numbers);


    // pop the last value
    numbers.pop();
    println!("Pop from numbers: {:?}", numbers);


    // for loop 
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // for loop mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Mutated numbers: {:?}", numbers);
}