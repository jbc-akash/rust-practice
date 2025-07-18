mod core_concepts;

fn main() {
    println!("-----------------------------------");
    println!("Rust practice Porject");
    println!("Running core concept exmaples...");
    println!("-----------------------------------");

    // core_concepts::ownership::demo_ownership();
    // core_concepts::borrowing::demo_borrowing();
    // core_concepts::traits::demo_traits();
    // core_concepts::collections::demo_collections();
    // core_concepts::structs_enums::demo_structs_enums();
    // core_concepts::generics::demo_generics();
    core_concepts::error_handling::demo_panic_macro();
}


// // ----------------------------------------------------------------------------
// // Associated Functions and Methods
// // Convert Data Type into the struct point
// #[derive(Debug)]
// struct Point {
//     x: u32,
//     y: u32,
// }

// // (u32, u32) -> Point
// impl From<(u32, u32)> for Point {
//     fn from(tuple: (u32, u32)) -> Self {
//         Point {
//             x: tuple.0,
//             y: tuple.1,
//         }
//     }
// }
// // ---------------------------------------------------------------------------



// /// Associated Type
// //  This is a placeholder for a trait that defines an associated type.
// // 1 implmentation per type
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// trait GenericIterator<T> {
//     fn get_next(&mut self) -> Option<T>;
// }

// struct ArrayIter<T> {
//     array: [T; 5],
//     i: usize,
// }

// impl GenericIterator<u32> for ArrayIter<u32> {
//     fn get_next(&mut self) -> Option<u32> {
//         match self.array.get(self.i) {
//             Some(value) => {
//                 self.i += 1;
//                 Some(*value)
//             }
//             _ => None,
//         }
//     }
// }

// fn main() {
//     let t: (u32, u32) = (19, 29);
//     let p = Point::from(t); // Associated Function also known as a static method]
//     println!("Point: {:?}", p);

//     let p: Point = t.into();
//     println!("Point: {:?}", p);
// }