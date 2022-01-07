// https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/crates-and-modules.html
extern crate phrases;

use phrases::chinese::{farewells, greetings};
use phrases::english;

fn main() {
    println!("Hello in English: {}", phrases::english::greetings::hello());
    println!(
        "Goodbye in English: {}",
        phrases::english::farewells::goodbye()
    );

    println!("Hello in Chinese: {}", greetings::hello());
    println!("Goodbye in Chinese: {}", farewells::goodbye());

    println!("Hello in English: {}", english::hello());
    println!("Goodbye in English: {}", english::goodbye());
}
