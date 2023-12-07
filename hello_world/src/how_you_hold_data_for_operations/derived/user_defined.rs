// This attribute allows dead code, which means code that is not used, to prevent warnings.
#![allow(dead_code)]

// Enumeration representing comparison operators
pub enum Comp {
    LessThan,
    GreaterThan,
    Equal,
}

// Enumeration representing gender
pub enum Gender {
    Male,
    Female,
}

// Struct representing a person with name and age
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Struct representing another person with name and age
struct Person2 {
    name: String,
    age: u8,
}

// Unit struct with no fields
struct Unit;

// Tuple struct with named fields
struct Pair(i32, f32);

// Struct representing a point in 2D space
struct Point {
    x: f32,
    y: f32,
}

// Struct representing a rectangle with top-left and bottom-right points
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// Trait representing a generic shape
trait Shape {
    // A method associated with the trait, taking a length parameter
    fn new(radius: f32) -> Self;

    // Another method to calculate the area of the shape
    fn area(&self) -> f32;
}

// Struct representing a circle implementing the Shape trait
struct Circle {
    radius: f32,
}

impl Shape for Circle {
    fn new(radius: f32) -> Self {
        Circle { radius }
    }

    fn area(&self) -> f32 {
        // Calculate the area of the circle (π * r^2)
        std::f32::consts::PI * self.radius * self.radius
    }
}

// Assume you have a struct Rect defined somewhere
struct Rect {
    width: f32,
    height: f32,
}

// Implementation of the Into trait for converting from Rect to Circle
impl Into<Circle> for Rect {
    fn into(self) -> Circle {
        // Let's create and return a Circle using some logic
        Circle {
            radius: self.width / 2.0,
        }
    }
}

fn main() {
    // Instantiate a Circle struct that implements the Shape trait
    let circle = Circle::new(5.0);

    // Print the area of the circle using the implemented method
    println!("Area of the circle: {}", circle.area());
}






