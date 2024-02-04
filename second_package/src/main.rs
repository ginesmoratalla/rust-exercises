// Clippy (format aider I think)
#![deny(clippy::all)]

use std::result;

fn main() {
    let message = return_string();
    println!("{}", message);

    let marcus = String::from("Marcus");
    let to_print = say_hello_world(String::from("Marcus"));
    println!("{}", to_print);

    let say_hello = |name: &str| format!("Good morning ,{}!", name);
    println!("{}", say_hello("World"));

    let name_surname = |name: &str, surname: &str| format!("Your full name: {} {}", name, surname);
    println!("{}", name_surname("Pedro", "Porro"));

    let multiply_by_2 = |x: i32| x * 2;
    let ptr = multiply_by_2;
    let result = ptr(20);
    println!("{}", result);
}

fn return_string() -> String {
    String::from("Kennedy")
}

fn say_hello_world(person: String) -> String {
    format!("Hello, {}!", person)
}

fn process_name(name: &str, callback: fn(&str) -> ()) {
    callback(name);
}
