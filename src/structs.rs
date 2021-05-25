// Traditional struct

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// tuple struct
struct TupleStruct(u8, u8, u8);


struct Person {
    first_name: String,
    last_name: String
}


impl Person {
    // construct person
    fn new(first_name: &str, last: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last.to_string()
        }
    }

    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_first_name(&mut self, first: &str) {
        self.first_name = first.to_string()
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}


pub fn run() {

    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut ct = TupleStruct(255, 0, 0);

    ct.0 = 200;

    println!("Color: {} {} {}", ct.0, ct.1, ct.2);


    let mut p = Person::new("Bilal", "Ibnu");

    println!("Person {} {}", p.first_name, p.last_name);
    println!("Full Name: {}", p.full_name());

    p.set_first_name("John");
    p.set_last_name("Doe");

    println!("Mutated person: {}", p.full_name());
    println!("To tuple: {:?}", p.to_tuple());
    
}