/*
Primitive types:
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, 64
Boolean (bool)
Characters (char)
Tuples
Arrays
 */

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the
// compiler can usually infer what type we want to use based on the value and how we used it.

pub fn run() {
    // Default is i32
    let _x = 1;
    // Default is f64
    let _y = 2.5;
    // Add explicit type
    let _z: i64 = 454545454;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;
    let is_greater: bool = 10 > 5;
    println!("{:?}", (is_active, is_greater));

    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (a1, face));
}
