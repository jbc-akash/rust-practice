use std::cmp::PartialOrd;

pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn coordinates(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

pub fn demo_generics() {
    println!(
        "Generics in Rust allow you to write flexible and reusable code by defining functions, structs, enums, and traits that can operate on different types without sacrificing type safety."
    );
    println!("\n=== Generics Concept Demo ===\n");

    // Excercise 1: Generic max function
    let int_max = max(42, 512);
    let float_max = max(3.14, 23.71);
    println!("Max of integers: {}", int_max);
    println!("Max of floats: {}", float_max);

    // Excercise 2: Generic Point Struct
    let int_point = Point { x: 1, y: 2 };
    let float_point = Point { x: 1.5, y: 2.5 };
    println!("Int point coordinates: {:?}", int_point.coordinates());
    println!("Float point coordinates: {:?}", float_point.coordinates());
}
