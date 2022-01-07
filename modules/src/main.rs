// extern crate phrases;

use modules::mod_a::mod_b;

// use phrases::english::farewells;
// use phrases::english::greetings;

fn main() {
    println!("Hello, world!");
    mod_b::test();

    // println!("Hello in English: {}", greetings::hello());
    // println!("Goodbye in English: {}", farewells::goodbye());
}
