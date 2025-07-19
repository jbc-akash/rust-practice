use std::f64::consts::PI;

/// A regular struct representing a rectangle.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

/// An enum representing various geometric shapes.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Shape {
    Circle(f64),         // radius
    Rectangle(f64, f64), // width, height
    Triangle(f64, f64),  // base, height
}

/// Describe the shape and calculate its area using pattern matching.
#[allow(dead_code)]
pub fn describe_shape(shape: &Shape) -> String {
    match shape {
        Shape::Circle(radius) => {
            let area = PI * radius * radius;
            format!("Circle with radius: {}, area: {:.2}", radius, area)
        }
        Shape::Rectangle(width, height) => {
            format!(
                "Rectangle with width: {} and height: {}, area: {:.2}",
                width,
                height,
                width * height
            )
        }
        Shape::Triangle(base, height) => {
            let area = 0.5 * base * height;
            format!(
                "Triangle with base {}, height {}, area: {:.2}",
                base, height, area
            )
        }
    }
}

/// A tuple struct representing a point in 2D space.
#[derive(Debug)]
pub struct Point(pub f64, pub f64);

/// A function to calculate the distance from a point to the origin (0, 0).
#[derive(Debug)]
pub struct Origin;

#[allow(unused_variables, dead_code)]
pub fn distance_to_origin(point: &Point, origin: &Origin) -> f64 {
    let dx = point.0 - 0.0; // x-coordinate of origin is 0
    let dy = point.1 - 0.0; // y-coordinate of origin is 0
    (dx * dx + dy * dy).sqrt()
}

/// An enum representing RGB colors.
#[derive(Debug)]
#[allow(dead_code)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[allow(dead_code)]
pub fn describe_shape_with_color(shape_color: &(Shape, Color)) -> String {
    let (shape, color) = shape_color;
    let color_str = match color {
        Color::Red => "Red",
        Color::Green => "Green",
        Color::Blue => "Blue",
    };
    let shape_desc = describe_shape(shape);
    format!("{} with color: {}", shape_desc, color_str)
}

/// A struct holding a string slice with its length using a lifetime.
#[derive(Debug)]
#[allow(dead_code)]
pub struct TextSlice<'a> {
    pub slice: &'a str,
    pub length: usize,
}

#[allow(dead_code)]
impl<'a> TextSlice<'a> {
    /// Create a new TextSlice from a string slice.
    pub fn new(slice: &'a str) -> Self {
        TextSlice {
            slice,
            length: slice.len(),
        }
    }

    pub fn truncate(&self, max_len: usize) -> TextSlice<'a> {
        let truncated = if self.slice.len() > max_len {
            &self.slice[..max_len]
        } else {
            self.slice
        };
        TextSlice {
            slice: truncated,
            length: truncated.len(),
        }
    }
}

/// A recursive enum reperesenting arithmetic expressions.
#[derive(Debug)]
#[allow(dead_code)]
pub enum Expression {
    Number(f64),
    Add(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
}

/// Evaluate an arithmetic expression to a number
#[allow(dead_code)]
pub fn evaluate(expr: &Expression) -> f64 {
    match expr {
        Expression::Number(value) => *value,
        Expression::Add(left, right) => {
            let left_value = evaluate(left);
            let right_value = evaluate(right);

            if left_value.abs() > 1e10 || right_value.abs() > 1e10 {
                panic!(
                    "Addition result too large | Overflow detected in addition: {} + {}",
                    left_value, right_value
                );
            }

            left_value + right_value
        }
        Expression::Multiply(left, right) => {
            let left_value = evaluate(left);
            let right_value = evaluate(right);

            if left_value.abs() > 1e10 || right_value.abs() > 1e10 {
                panic!(
                    "Multiplication result too large | Overflow detected in multiplication: {} * {}",
                    left_value, right_value
                );
            }

            left_value * right_value
        }
    }
}

#[allow(dead_code)]
pub fn demo_structs_enums() {
    println!(
        "Structs and enums in Rust allow you to create custom data types that can encapsulate related data and behavior."
    );
    println!("\n=== Structs and Enums Concept Demo ===\n");

    // Exercise 1: Reactangle struct
    let rect = Rectangle {
        width: 5.0,
        height: 3.0,
    };

    println!("Rectangle area: {}", rect.area());

    let square = Rectangle::square(4.0);
    println!("Square area: {}", square.area());

    //Exercise 2: Shape enum
    let circle = Shape::Circle(2.0);
    let rectangle = Shape::Rectangle(6.0, 4.0);
    println!("Circle description: {}", describe_shape(&circle));
    println!("Rectangle description: {}", describe_shape(&rectangle));

    // Excercise 3: Point and Origin
    let point = Point(3.0, 4.0);
    let origin = Origin;
    println!(
        "Distance from point {:?} to origin {:?} is {}",
        point,
        origin,
        distance_to_origin(&point, &origin)
    );

    // Exercise 4: Shape with Color
    let colored_circle = (Shape::Circle(1.5), Color::Red);
    let colored_triangle = (Shape::Triangle(3.0, 4.0), Color::Blue);
    println!(
        "Colored circle: {}",
        describe_shape_with_color(&colored_circle)
    );
    println!(
        "Colored triangle: {}",
        describe_shape_with_color(&colored_triangle)
    );

    // Excercise 5: TextSlice
    let text = String::from("Hello, Rust!");
    let text_slice = TextSlice::new(&text);
    println!(
        "TextSlice: {} (length: {})",
        text_slice.slice, text_slice.length
    );
    let truncated = text_slice.truncate(5);
    println!(
        "Truncated TextSlice: {} (length: {})",
        truncated.slice, truncated.length
    );

    // Exercise 6: Expression enum
    let expr = Expression::Add(
        Box::new(Expression::Number(3.0)),
        Box::new(Expression::Multiply(
            Box::new(Expression::Number(2.0)),
            Box::new(Expression::Number(4.0)),
        )),
    );

    println!("Expression: {:?}", expr);
    let result = evaluate(&expr);
    println!("Evaluated result: {}", result);
}
