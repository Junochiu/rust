pub fn run() {
    // print to console
    println!("Hello World?");
    println!("Number: {}", 1);
    println!("Printing two parameters: {}, {}", 1, "two");
    // positional parameter
    println!("{0}, {1}, {2}, {0}, {1}, {2}", "a", "b", "c");
    // named parameter
    println!(
        "{name} likes to {activity}",
        name = "Juno",
        activity = "travel"
    );
    // placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);
    // placeholder debug traits
    println!("{:?}", ("debug1", 1, 2));
    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
