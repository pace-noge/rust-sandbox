pub fn run() {
    greeting("Hello", "nasa");

    let get_sum = add(5, 4);

    println!("{}", get_sum);

    // closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;

    println!("C sum: {}", add_nums(3, 3));
    
    // Closure with variable outside of closure
    let another_sums = |x: i32, y: i32| x + y + get_sum;

    println!("another sum {}", another_sums(2, 3));

}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}