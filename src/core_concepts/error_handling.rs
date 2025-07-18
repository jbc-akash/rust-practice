/// Demonstrates the `panic!` macro in Rust.
#[allow(dead_code)]
pub fn demo_panic_macro() {
    println!("=== Panic Macro Demo ===\n");

    println!(
        "In Rust, the `panic!` macro is used to immediately stop program execution \
        when an unrecoverable error occurs. It prints an error message and unwinds \
        the stack (or aborts, depending on settings)."
    );

    let condition = false;

    // If the condition is false, trigger a panic with a custom message.
    if !condition {
        panic!("Panic occurred because `condition` was false.");
    }

    println!("This line will never be reached if panic occurs.");
}

/// Demonstrates error handling and panic tracing in Rust.
#[allow(dead_code)]
pub fn demo_error_handling() {
    println!("=== Error Handling Demo ===\n");

    println!(
        "Rust handles errors through `Result` and `Option` types instead of exceptions. \
        For unrecoverable errors, `panic!` is used.\n"
    );

    println!("Now demonstrating a panic with backtrace:\n");
    
    // Uncomment the line below to see a simple panic example
    // demo_panic_macro();

    demo_backtrace();
}


/// Demonstrates panic backtrace using nested function calls.
#[allow(dead_code)]
pub fn demo_backtrace() {
    println!("=== Backtrace Demo ===\n");

    println!(
        "When a panic occurs, Rust can print a backtrace showing the call stack. \
        Set the environment variable `RUST_BACKTRACE=1` to see the backtrace."
    );

    a(); // Start the chain of calls leading to panic.
}

/// First function in the call chain.
pub fn a() {
    b();
}

/// Second function in the call chain.
pub fn b() {
    c(22); // Will panic here if number is 22
}

/// Third function in the call chain that triggers panic.
pub fn c(num: i32) {
    if num == 22 {
        panic!("Panic in function `c`: received number {}", num);
    } else {
        println!("Number is not 22, execution continues.");
    }
}