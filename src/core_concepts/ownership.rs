pub fn demo_ownership() {
    println!("Ownership in Rust is a set of rules that governs how memory is managed.");
    println!("1. Each value in Rust has a variable that is its 'owner'.");
    println!("2. A value can only have one owner at a time.");
    println!("3. When the owner goes out of scope, the value will be dropped (memory freed).");
    println!("4. Ownership can be transferred (moved) to another variable, but the original variable can no longer be used.");
    println!("5. Rust also has references, which allow you to borrow a value without taking ownership."); 


    // Example: Simple ownership transfer
    let s1 = String::from("Hello, Rust!");
    let s2 = s1; // Ownership of the string is moved from s1 to s2
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // This line would cause a compile-time error because s1 is no longer valid

    // Example: Copy for primitive types
    let x = 5; // i32 is a Copy type
    let y = x; // Ownership is not moved, x is still valid
    println!("x: {}, y: {}", x, y); // Both x and y can be used
}