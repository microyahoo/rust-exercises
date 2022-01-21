fn main() {
    println!("Hello, world!");
}

// ================================================================================================
// https://stackoverflow.com/questions/32304595/whats-the-difference-between-self-and-self
pub trait MyTrait {
    fn to_larger(self) -> i64;
    fn add_one(&mut self);
    fn new() -> Self;
}

impl MyTrait for i32 {
    fn to_larger(self) -> i64 {
        // This function takes ownership of self, meaning self is effectively destroyed when this function runs.
        1
    }
    fn add_one(&mut self) {
        // This function takes a mutable reference, it can modify itself, but will continue to live after this function
    }
    fn new() -> Self {
        // Self means the current type, which is i32. This function returns something that is an i32.
        1
    }
}

// self refers to the current module or object.
// Self refers to the type of the current module or object.
struct Employee {
    name: String,
}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
