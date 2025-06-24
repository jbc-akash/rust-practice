// Traits: Define shared behavior across types, similar to interfaces in other languages.
// Trait bounds: Specify that a generic type must implement a certain trait.
pub trait Summary {
    fn summarize(&self) -> String;

    fn default_summary(&self) -> String {
        String::from("No summary available.")
    }
}

// Struct 1: Tweet
#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn default_summary(&self) -> String {
        format!("Tweet from @{}", self.username)
    }
}

// Struct 2: Article
#[derive(Debug)]
pub struct Article {
    pub title: String,
    pub author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

// Struct 3: Book - implments default method only
#[derive(Debug)]
pub struct Book {
    pub name: String,
    pub writer: String,
}

impl Summary for Book {
    fn summarize(&self) -> String {
        format!("Book: {} by {}", self.name, self.writer)
    }
}

// Trait: Printable - extra trait for multiple bounds demo
pub trait Printable {
    fn print(&self);
}

impl Printable for Tweet {
    fn print(&self) {
        println!(
            "Tweet => User: {}, Content: {}",
            self.username, self.content
        );
    }
}

impl Printable for Article {
    fn print(&self) {
        println!("Article => Title: {}, Author: {}", self.title, self.author);
    }
}

// Generic function with single trait bound
pub fn print_summary<T: Summary>(item: &T) {
    println!("Summary: {}", item.summarize());
}

// Generic function with multiple trait bounds
pub fn display_details<T: Summary + Printable>(item: &T) {
    println!("\n-- Displaying Details --");
    item.print();
    println!("Summary: {}", item.summarize());
    println!("Default Summary: {}", item.default_summary());
    println!("-------------------------\n");
}

pub fn demo_traits() {
    // Objective: Understand traits, trait bounds, and trait implementations to achieve polymorphism in Rust.
    println!(
        "Traits in Rust are a way to define shared behavior across types, similar to interfaces in other languages."
    );
    println!("\n=== Trait Concept Demo ===\n");

    // Exercise 1: Define a trait and implement it for a struct
    let tweet = Tweet {
        username: String::from("Akash"),
        content: String::from("Learning Rust"),
    };

    let article = Article {
        title: String::from("Rust programming"),
        author: String::from("Md Mostafa Akash"),
    };

    let book = Book {
        name: String::from("The Rust Book"),
        writer: String::from("Steve Klabnik"),
    };

    // Exercise 1: Print summary using trait bound
    println!("--- Basic Summary ---");
    print_summary(&tweet);
    print_summary(&article);
    print_summary(&book);

    // Exercise 2: Use multiple traits
    display_details(&tweet);
    display_details(&article);

    // Book doesn’t implement Printable, so can't be passed to `display_details`
    println!("Book default summary: {}\n", book.default_summary());
}
