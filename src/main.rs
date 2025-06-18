mod core_concepts;

fn main() {
    println!("Rust practice Porject");
    println!("Running core concept exmaples...");

    core_concepts::ownership::demo_ownership();
    core_concepts::borrowing::demo_borrowing();
}
