/*
Primitive types ---
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they tkae in memory) u -> unsigned (non negative), i-> can be pos or neg
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is statically typed language.


pub fn run() {
    // default i32;
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 4545454545445;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Min i32: {}", std::i32::MIN);
    println!("Max u32: {}", std::u32::MAX);
    println!("Min u32: {}", std::u32::MIN);
    println!("Max i64: {}", std::i64::MAX);


    // Boolean
    let is_active: bool = true;

    // Boolean from expression
    let is_greater: bool = 10 < 2;

    // Char
    let a1 = 'a';

    // emoji
    let face = '\u{1f600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));

}