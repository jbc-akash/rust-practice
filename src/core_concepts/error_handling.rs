
pub fn demo_panic_macro() {
    // The panic! macro is used to stop execution and indicate an unrecoverable error.
    println!(
        "The panic! macro in Rust is used to stop execution and indicate an unrecoverable error. 
        It can be used with a message to provide context."
    );
    println!("\n======== Panic Macro Demo =============\n");

    // Example of using panic! macro
    let condition = false;
    if !condition {
        panic!("This is a panic message: Condition was false!");
    }
}