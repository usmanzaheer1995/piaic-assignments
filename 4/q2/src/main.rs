fn different_arguments(int: i32, float: f32, boolean: bool) {
    println!("Entered integer: {}", int);
    println!("Entered float: {}", float);
    println!("Entered boolean: {}", boolean);
}

fn main() {
    different_arguments(42, 32.0, true);
}
