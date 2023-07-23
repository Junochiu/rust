// Structs - Used to create custom data types

// Tradition struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct TupleColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

// function that's associate with the person
impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("Color {} {} {}", c.red, c.green, c.blue);
    c.red = 200;
    println!("Color {} {} {}", c.red, c.green, c.blue);

    let mut c1: TupleColor = TupleColor(255, 0, 0);
    println!("Tuple color: {} {} {}", c1.0, c1.1, c1.2);

    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Fullname {}", p.full_name());
    p.set_last_name("foo");
    println!("Fullname {}", p.full_name());

    println!("Person tuple {:?}", p.to_tuple());
}
