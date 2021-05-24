pub fn run() {
    let name = "Bilal";
    let mut age = 36;

    println!("My name is {} and i am {}", name, age);
    
    age = 38;

    println!("My name is {} and i am {}", name, age);

    // define constant
    const ID: i32 = 001;

    println!("ID: {id}", id=ID);

    // assign multiple vars
    let (my_name, my_age) = ("Bilal", 2);

    println!("{} is {}", my_name, my_age);
}