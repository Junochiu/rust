// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let hello = "testing"; //str
    let mut hello2 = String::from("testing ");
    println!("{:?}", (hello, hello2.clone()));

    // Get length
    println!("Length: {}", hello.len());

    // hello.push('\u{1F600}'); cannot work
    hello2.push('\u{1F600}');
    hello2.push_str("\u{1F600}\u{1F600}\u{1F600}");
    println!("{}", hello2);

    // Capacity
    println!("Capacity: {}", hello2.capacity());
    // Is empty
    println!("Is Empty: {}", hello.is_empty());
    // Contains
    println!("Contains \u{1F600}: {}", hello2.contains("\u{1F600}"));
    // Replace
    println!(
        "Replace \u{1F600}: {}",
        hello2.replace("\u{1F600}", "\u{1F602}")
    );
    // Loop through string by whitespace
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }
    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    // Assertion testing
    assert_eq!(3, s.len());
    println!("{}", s);
}
