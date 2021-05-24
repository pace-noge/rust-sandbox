// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when need to modify data


pub fn run() {

    let mut hello = String::from("Hello");


    // Get length
    println!("Length: {}", hello.len());


    hello.push_str(" World");

    println!("{}", hello);

    // is empty
    println!("is empty {}", hello.is_empty());

    // Capacity in bytes
    println!("Capacity {}", hello.capacity());

    // contains substr
    println!("Contains 'World' {}", hello.contains("World"));

    // replace
    println!("Replace: {}", hello.replace("World", "There"));

    // loop through string
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

}