# Rust Practice Project

This project is a collection of Rust exercises to build a solid foundation in Rust programming. It covers core Rust concepts (ownership, borrowing, lifetimes, traits, etc.).

## Project Structure
- `src/core_concepts/`: Exercises for learning Rust fundamentals.
- `src/tests/`: Unit and integration tests for all exercises and solutions.

## Getting Started
1. Install Rust: Follow instructions at [rust-lang.org](https://www.rust-lang.org/).
2. Clone this repository: `git clone <repo-url>`.
3. Navigate to the project directory: `cd rust_practice`.
4. Run the project: `cargo run`.
5. Run tests: `cargo test`.

## Adding New Problems
- For core concepts, create a new file in `src/core_concepts/` and add a module in `core_concepts/mod.rs`.
- For LeetCode problems, create a new file in `src/leetcode/` and add a module in `leetcode/mod.rs`.
- Write tests for each problem in `src/tests/` under the appropriate test module.

## Example Usage
```bash
cargo run
# Output: Running core concept examples and LeetCode solutions...
```

## 📘 Core Rust Concepts Included
Below are the key Rust concepts covered in this repo under src/core_concepts/:

| Concept                  | Description                                                                 |
|--------------------------|-----------------------------------------------------------------------------|
| ✅ Ownership              | How Rust manages memory through ownership rules.                           |
| ✅ Borrowing & References | Shared access to data via `&T` and exclusive access via `&mut T`.           |
| ✅ Traits                 | Defining and implementing shared behavior (similar to interfaces).          |
| ✅ Trait Bounds           | Generics with constraints using `T: Trait`.                                 |
| ✅ Multiple Trait Bounds  | Using `+` syntax for compound trait requirements.                           | 
| ✅ Vec              | Growable, indexable list of items (`Vec<T>`)                                      |
| ✅ HashMap          | Unordered key-value store, great for counting, mapping                            |
| ✅ HashSet          | Unordered set of unique values                                                    |
| ✅ VecDeque         | Double-ended queue with fast push/pop at both ends                                |
| ✅ BTreeMap         | Ordered key-value map (sorted by key) | 

📝 All concepts include examples with comments and often test cases

## Contributing
Feel free to add new problems or improve existing solutions. Ensure all code is tested and follows Rust best practices.