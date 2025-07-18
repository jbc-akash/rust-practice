#[allow(dead_code)]
pub fn demo_ownership() {
    println!("Ownership in Rust is a set of rules that governs how memory is managed.");

    let s3 = String::from("Hello");
    let s3_new = transfer_ownership(s3); // Ownership of s3 is moved to transfer_ownership function
    println!("s3_new: {}", s3_new);
    // s3 is no longer valid here
    // println!("s3: {}", s3); // This line would cause a compile-time error because s3 is no longer valid

    let x = 42;
    let (original, copied) = copy_value(x); // x is copied, original and copied can both be used
    println!("original: {}, copied: {}", original, copied);
}

#[allow(dead_code)]
pub fn transfer_ownership(s: String) -> String {
    s // Ownership is returned to the caller
}

#[allow(dead_code)]
pub fn copy_value(x: i32) -> (i32, i32) {
    let y = x; // x is copied to y
    (x, y) // Both x and y can be used after this
}
