mod core_concepts;

fn main() {
    println!("-----------------------------------");
    println!("Rust practice Porject");
    println!("Running core concept exmaples...");
    println!("-----------------------------------");



    core_concepts::ownership::demo_ownership();
    core_concepts::borrowing::demo_borrowing();
    core_concepts::traits::demo_traits();
}
