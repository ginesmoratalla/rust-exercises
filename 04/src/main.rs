// Clippy (format aider I think)
#![deny(clippy::all)]


// Enums
#[derive(Debug, PartialEq, Eq)]
enum AnimalType {
    Dog,
    Cat,
    Rabbit,
}

struct Size {
    width: f32,
    height: f32,
}
//#[derive(Debug, PartialEq, Eq)]
enum Shape {
    Circle {radius: f64, center: (f64, f64)},
    Rectangle {width: f64, height: f64,},
    Rectangle2(f32, f32, Size),
    Circle2(f32, f32, f32),

}
impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Rectangle2(x, y, size) => size.width * size.height,
            Shape::Circle2(x, y, radius) => 3.14 * (radius * radius),
            _ => {println!("no"); 0.0}
        }
    }
}

enum Pets {
    Cat {name: String}, 
    Dog {name: String},
}


fn main() {

    let fluffy = AnimalType::Dog;

    // Line 6 import necessary for this
    if fluffy == AnimalType::Dog {
        println!("Fluffy is a Dog");
    }

    // Rust switch case (Match)
    match fluffy {
        AnimalType::Cat => print!("Gato"),
        AnimalType::Dog => print!("Perro"),
        AnimalType::Rabbit => print!("Conejo"),
    }

    // Switch with only one condition (catch all)
    match  fluffy {
        AnimalType::Cat => print!(" is drinking water"),
        _ => print!(" is not drinking water")
    }

    let rectangle = Shape::Rectangle { width: (3.0), height: (4.0) };
    if let Shape::Rectangle { width, height } = rectangle {
        println!("\nRectangles width is {} and height is {}", width, height);
    }

    // Combination of tuple and structure
    let rectangle2 = Shape::Rectangle2(1.0, 2.0, Size{width: 3.0, height: 4.0});

    if let Shape::Rectangle2(x, y, Size {width, height}) = rectangle2 {
        println!("{}, {}, {}, {}", x, y, width, height);
    }

    match rectangle2 {
        Shape::Rectangle2(x, y, Size{width, height}) => println!("YOU WON THE LOTTERY"),
        _ => println!("loser"),
    }
    println!("Rectangle area is: {}", rectangle2.area());

    // Match is an expression, it can return or evaluate a value
    let pet = Pets::Cat { name: "Fluffy".to_string() };
    let name = match pet {
        Pets::Cat { name } => name,
        Pets::Dog { name } => name,
    };

    println!("My pets name is: {}", name);

}
