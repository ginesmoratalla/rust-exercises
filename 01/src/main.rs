// Clippy (format aider I think)
#![deny(clippy::all)]

const MY_AGE: u8 = 22;

fn main() {
    println!("Hello, world");

    // With let keyword we declare variables immutable (can't change)
    let x = "John";
    let y = "Marcus";

    let _var = 1;

    println!("{} {}", x, y);

    // Variables use snake_case naming convention (e.g. first_name, NOT firstName)
    // https://doc.rst-lang.org/1.0.0/style/style/naming/README.md

    // Try to change John's name
    // x = "Michael" (this won't work)
    // let x = "Michael"; (this will work "shadowing")

    // Declare mutable variable (can change the data inside, but not the data type)
    let mut name = "Pedro";
    println!("{}", name);
    name = "Chris";
    println!("{}", name);

    // Specify data types
    let age: u32 = 30;
    let age2 = 30u8;

    // Separating zeros from a variable with underscores or other formats
    let population = 62_000_000;
    let hex = 0xFA;
    let rgb = 0xFF0000;
    let distance: f64 = 5.0;
    let distance2: f64 = 6.0;
    let total_distance = distance + distance2;

    // Variable shadowing is allowed, even different data type
    let nam = "Yop";
    let nam = 10;
    {
        let nam = nam.to_string();
        println!("{}", nam);
    }

    println!("CONSTANT: {}", MY_AGE);

    // Tupples
    let personal_data = (12, "Pedro");
    // Define the names of the data inside a tupple
    let (age, name) = personal_data;
    let age_student = personal_data.0;
    println!("This is {}, he is {}", name, age);

    // Ownership
    let name1 = String::from("John");
    let name2 = name1;

    //println!("{} and {}", name1, name2);
    // This won't work, because name1 was borrowed (read only) by name2
    // the stack and heap space form name1 is allocated to name2 after the "borrowing" happens

    {
        let namee = "John".to_string();
        println!("{}", namee);
    }

    /* Stack vs Heap
     *
     * A string in rust is stored both in the stack and the heap
     * Therefore, moving will happen, changing the ownership of variables like the name1
     * and name2 explained above
     */

    // References in rust
    let s1 = String::from("James Bond");
    let s2 = &s1;
    println!("My firends {} and {}", s1, s2);
    greet(&s1);

    // Modifying mutable references
    let mut test_string = String::from("Khaled");
    empty_string(&mut test_string);

    // You cannot have more than 1 mutable reference to a variable
    let mut test_string1 = String::from("Khaled");
    let mut test1 = &mut test_string1;
    let mut test2 = &mut test_string1;
    //empty_string(&mut test1);

    // Cannot have mutable references while immutable references exist
    let mut other1 = String::from("Jo");
    let nn = &other1;
    let mut nn2 = &mut other1;
    //println!("{}", nn);

    // Scope of variables is very

    let message = return_string();
    println!("{}", message)

    // Dangling references
}

fn greet(name: &String) {
    println!("You are this person: {}", name);
}

fn empty_string(string: &mut String) {
    string.clear();
}

fn get_name() -> String {
    "John".to_string()
}

fn return_string() -> String {
    String::from("Kennedy")
}
