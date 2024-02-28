// Clippy (format aider I think)
#![deny(clippy::all)]

use std::vec;

// Tuples: unnamed groups of data
struct Point(f32, f32);


fn get_values() -> (String, String, i32) {
    ("Yap".to_string(), "Yaaap".to_string(), 30)
}
fn main() {

    let values = ("Hello", "World", 30);
    let hello = values.0;

    // Packed all of the values from values onto three variables
    let(one, two, three) = values;
    let(_,_,age) = get_values();
    println!("{}", hello);

    // Vectors
    let vector: [&str; 2] = ["faa", "foo"];
    let foo = &vector[0];
    for val in vector.iter() {
        println!("{}", val)
    }
    let length = vector.len();

    // Mapping
    let vector2: [i32; 2] = [20, 10];
    let double_vector2 = vector2.iter().map(|x| x * 32);

    // Other way of declaring vector
    let mut values3 = vec![1, 2, 3];
    values3.push(4);
    let four = values3.pop();

    // Remove values from a mutable vector
    println!("Values: {:?}", values3);
    values3.extend_from_slice(&[1, 1, 1]);
    values3.clear();
    println!("Values: {:?}", values3);

    // Clone vector
}
