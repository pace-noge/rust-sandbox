// Enums are types whic have definite value

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}


fn move_avatar(m: Movement) {
    // perform action depending on info
    match m {
        Movement::Up => println!("UP"),
        Movement::Down => println!("DOWN"),
        Movement::Left => println!("LEFT"),
        Movement::Right => println!("RIGHT")
    }
}

pub fn run() {
    let up = Movement::Up;
    let down = Movement::Down;
    let left = Movement::Left;
    let right = Movement::Right;

    move_avatar(up);
    move_avatar(down);
    move_avatar(left);
    move_avatar(right);
}