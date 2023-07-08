// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a  block-scoped lauguage

pub fn run() {
    // immutable
    let name = "variable";
    // mutable
    let mut age = 24;
    println!("myname is {}, age {}.", name, age);
    age = 25;
    println!("myname is {}, age {}.", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let (my_name, my_age) = ("juno", 24);
    println!("{} is age {}", my_name, my_age);
}
