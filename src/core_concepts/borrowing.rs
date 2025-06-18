pub fn demo_borrowing() {
    println!(
        "Borrowing in Rust allows you to use a value without taking ownership of it, enabling multiple references to the same data."
    );

    println!("Example of borrowing a value:");
    let s1 = String::from("Hello");
    let len = calculate_length(&s1); // Borrowing s1 by passing a reference
    println!("The length of '{}' is {}.", s1, len);

    println!("Example of mutable borrowing:");
    let mut s2 = String::from("Hello");
    append_word(&mut s2, " world"); // Mutably borrowing s2 to modify it
    println!("After appending, s2 is '{}'.", s2);
}

pub fn calculate_length(s: &String) -> usize {
    s.len() // Using the borrowed reference to calculate length
}

pub fn append_word(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}
