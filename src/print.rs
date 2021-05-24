pub fn run() {
    // print to console
    println!("Hello from print file");

    println!("{}", 1);

    // basic formatting
    println!("{} is from  {}", "brad", "Mass");

    // Positional Argumensts
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "Code");


    // named arguments
    println!("{name} is from  {planet} and {name} likes to {likes}",
        name="Brad",
        planet="Mars",
        likes="Code"
    );


    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}