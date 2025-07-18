use std::cmp::PartialOrd;
use std::fmt::Display;

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

// Struct with two different generic types
#[derive(Debug)]
#[allow(dead_code)]
pub struct Pair<T, U> {
    pub left: T,
    pub right: U,
}

// Generic Enum
#[derive(Debug)]
pub enum ResultLike<T, E> {
    Ok(T),
    Err(E),
}

pub fn print_and_return_max<T>(a: T, b: T) -> T 
where
    T: PartialOrd + Display,
    {
        let max_value = if a > b { a } else { b };
        println!("Max value: {}", max_value);
        max_value
    }
    
#[allow(dead_code)]
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


    // Excercise 3: Pair struct with two types
     let pair = Pair { left: "left", right: 100 };
     println!("Pair: {:?}", pair);

     // Excercise 4: Generic Enum
     let ok: ResultLike<i32, &str> = ResultLike::Ok(10);
     let err: ResultLike<i32, &str> = ResultLike::Err("Error occurred");
     println!("ResultLike Ok: {:?}", ok);   
     println!("ResultLike Err: {:?}", err);

     // Excercie 5: Generic function with multiple trait bounds
     print_and_return_max("hellosadfa", "worlds");
     print_and_return_max(10, 20);
}
