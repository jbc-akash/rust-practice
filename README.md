# Rust Practice Project

This project is a collection of Rust exercises and LeetCode problem solutions to build a solid foundation in Rust programming. It covers core Rust concepts (ownership, borrowing, lifetimes, traits, etc.) and organizes LeetCode problems by category.

## Project Structure
- `src/core_concepts/`: Exercises for learning Rust fundamentals.
- `src/leetcode/`: Solutions to LeetCode problems, organized by problem type or number.
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

## Contributing
Feel free to add new problems or improve existing solutions. Ensure all code is tested and follows Rust best practices.