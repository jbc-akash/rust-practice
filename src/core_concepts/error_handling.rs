#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn demo_error_handling() {
    // Objective: Understand error handling in Rust using Result and Option types.
    println!(
        "Error handling in Rust is done using the Result and Option types, which provide a way to handle errors without exceptions."
    );
    println!("\n======== Error Handling Demo =============\n");

    // demo_panic_macro();
    demo_backtrace();
}

#[allow(dead_code)]
pub fn demo_backtrace() {
    // The backtrace feature provides a way to see the call stack when a panic occurs.
    println!(
        "The backtrace feature in Rust provides a way to see the call stack when a panic occurs. 
        It can be enabled with the RUST_BACKTRACE environment variable."
    );
    println!("\n======== Backtrace Demo =============\n");

    a();
}

pub fn a() {
    b();
}

pub fn b() {
    c(22);
}

pub fn c(num: i32) {
    if num == 22 {
        panic!("This is a panic in function c with number: {}", num);
    } else {
        println!("Number is not 22, continuing execution.");
    }
}