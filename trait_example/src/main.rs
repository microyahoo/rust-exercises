fn main() {
    let c: Cat = Animal::new("Bobi");
    println!("Cat's name is {} {} ", c.name(), c.age);
    c.talk();

    let stuffed: StuffedAnimal = Animal::new("BobiStuffed");
    stuffed.talk();
}

// ================================================================================================

// ================================================================================================

// ================================================================================================

// ================================================================================================
trait Animal {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Cat {
    name: &'static str,
    age: i32,
}

struct StuffedAnimal {
    name: &'static str,
}

impl Animal for Cat {
    fn new(name: &'static str) -> Cat {
        Cat { name: name, age: 1 }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "Meowww"
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

impl Animal for StuffedAnimal {
    fn new(name: &'static str) -> StuffedAnimal {
        StuffedAnimal { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "<random factory noise>"
    }
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
