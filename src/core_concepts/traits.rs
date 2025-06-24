pub fn demo_traits() {
    // Objective: Understand traits, trait bounds, and trait implementations to achieve polymorphism in Rust.
    println!("Traits in Rust are a way to define shared behavior across types, similar to interfaces in other languages.");
    println!("Traits Demo:");

    // Exercise 1: Define a trait and implement it for a struct
    let tweet = Tweet {
        username: String::from("akash"),
        content: String::from("Learning Rust"),
    };

    println!("Tweet: {}", tweet.summarize());

    // Exercise 2: Generic print summary function
  
    let article = Article {
        title: String::from("Rust programming"),
        author: String::from("Md Mostafa Akash"),
    };

    print_summary(&tweet); 
    print_summary(&article); 


}

pub trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
}

#[derive(Debug)]
pub struct Article {
    pub title:  String,
    pub author: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

pub fn print_summary<T: Summary>(item: &T) {
    println!("Summary: {}", item.summarize());
}

