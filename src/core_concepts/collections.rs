use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

/// Count the frequency of words using a HashMap.
/// If a word appears multiple times, its count increases.
pub fn count_word_frequencies(words: Vec<String>) -> HashMap<String, usize> {
    let mut freq = HashMap::new();

    for word in words {
        *freq.entry(word).or_insert(0) += 1;
    }

    freq
}

/// Collect unique words using a HashSet.
/// Duplicates are automatically discarded.
#[allow(dead_code)]
pub fn get_unique_words(words: Vec<String>) -> HashSet<String> {
    let mut unique_words = HashSet::new();

    for word in words {
        unique_words.insert(word);
    }

    unique_words
}

#[allow(dead_code)]
pub fn demo_collections() {
    println!("\nCollections in Rust are used to store multiple values in a single data structure.");
    println!("=== Rust Collections Demo ===\n");

    //Excercise 1: Count frequencies of words in a string using a HashMap
    let words = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("apple"),
        String::from("orange"),
        String::from("banana"),
        String::from("apple"),
        String::from("kiwi"),
    ];

    let freq = count_word_frequencies(words.clone());
    println!("Word Frequencies: {:?}", freq);

    // Excercise 2: Unique elements using a HashSet
    let unique_words = get_unique_words(words);
    println!("Unique Words: {:?}", unique_words);

    // Excercise 3: Using BTreeMap for sorted key-value pairs
    let mut scores = BTreeMap::new();
    scores.insert(String::from("Alice"), 90);
    scores.insert(String::from("Bob"), 85);
    scores.insert(String::from("Charlie"), 95);
    scores.insert(String::from("Alive"), 80);
    scores.insert(String::from("BAF"), 95);

    println!("Scores (BTreeMap): {:?}", scores);

    // Excercise 4: Using VecDeque for efficient queue operations
    let mut dequeue: VecDeque<String> = VecDeque::new();
    dequeue.push_back(String::from("First"));
    dequeue.push_back(String::from("Second"));
    dequeue.push_front(String::from("Zero"));

    println!("Deque after push operations: {:?}", dequeue);
    if let Some(front) = dequeue.pop_front() {
        println!("Popped from front: {}", front);
    }

    println!("Deque after pop from front: {:?}", dequeue);
    if let Some(back) = dequeue.pop_back() {
        println!("Popped from back: {}", back);
    }
    println!("Deque after pop from back: {:?}", dequeue);
    println!("-----------------------------------");
}
