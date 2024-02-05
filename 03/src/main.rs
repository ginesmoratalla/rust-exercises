#![deny(clippy::all)]

// Structures 
struct Person {
    name: String,
    age: u8,
    mothers_name: String
}

// Add debug trait to Point tuple
#[derive(Debug)]
struct Point(f64, f64, f64);

// Implementation
// Functions inside here are created in every point instance
impl Point {
    fn describe(&self) {
        println!("Point at:  x = {}; y = {}; z = {}", self.0, self.1, self.2);
    }
    fn doubled(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }
    fn make_twice(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }

    // Non-method associated functions
    fn make_point_zero() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}

fn create_person(name: String, age: u8) {
    // both vars match the name of the original structure
    // therefore, no need to name: name
    //let person = Person {name, age};
}

// Structure Update syntax
// Create instances of structures with predefined field values in case of repeating

fn main() {

    let person = Person {
        name: "Marcos".to_string(),
        age: 19,
        mothers_name: "Jane".to_string()
    };

    println!("Name: {}, Age: {}",person.name, person.age);
    let person2 =  Person {
        name: "Doe".to_string(),
        // VALID: age: person.age
        ..person
    };

    let point = Point(15.0, 20.0, 30.5);
    point.describe();
    let mut new = point.doubled();
    new.describe();
    new.make_twice();    
    new.describe();


    // Debugging instance of point
    print!("DEBUG: ");
    println!("{:?}", new);

    // Non-method associated calls
    let point1 = Point::make_point_zero();
    point1.describe()

}
