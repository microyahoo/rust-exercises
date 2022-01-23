use trace_caller::trace;

fn main() {
    let c: Cat = Animal::new("Bobi");
    println!("Cat's name is {} {} ", c.name(), c.age);
    c.talk();

    let stuffed: StuffedAnimal = Animal::new("BobiStuffed");
    stuffed.talk();

    println!("=================================================");
    // https://users.rust-lang.org/t/can-not-understand-temporary-value-dropped-while-borrowed/23279/7
    // ERR: create a reference to a temporary that has been dropped.
    // let foo = "FooBar".to_string().as_mut_str();
    // println!("{}", foo);

    // let foo = {
    //     let mut __temp = "FooBar".to_string();
    //     __temp.as_mut_str()
    // };
    // println!("{}", foo);

    println!("=================================================");
    // https://stackoverflow.com/questions/28587698/whats-the-difference-between-placing-mut-before-a-variable-name-and-after-the
    // https://stackoverflow.com/questions/29672373/what-is-difference-between-mut-a-t-and-a-mut-t
    let f1 = FullName {
        first_name: String::from("Jobs"),
        last_name: String::from("Steve"),
    };

    // mut a: &T
    let mut a = &f1;
    println!("{}: {}", a.last_name, a.first_name);

    let f2 = FullName {
        first_name: String::from("Gates"),
        last_name: String::from("Bill"),
    };
    // a 重新绑定到一个新的 FullName 的引用
    a = &f2;

    // 不允许对 a 指向的内容作出修改
    // a.first_name = String::from("Error"); // `a` is a `&` reference, so the data it refers to cannot be written

    println!("{}: {}", a.last_name, a.first_name);

    println!("=================================================");
    let mut f1 = FullName {
        first_name: String::from("Rust"),
        last_name: String::from("The programming language"),
    };
    // mut a: &mut T
    let a = &mut f1;
    println!("{}: {}", a.last_name, a.first_name);
    a.first_name = String::from("Golang");
    println!("{}: {}", a.last_name, a.first_name);

    // let mut f2 = FullName {
    //     first_name: String::from("Python"),
    //     last_name: String::from("The programming language"),
    // };
    // a = &mut f2; // [rustc E0384] [E] cannot assign twice to immutable variable `a`
    // println!("{}: {}", a.last_name, a.first_name);
    println!("=================================================");
    println!("=================================================");
}

// ================================================================================================

// ================================================================================================
struct FullName {
    first_name: String,
    last_name: String,
}

// ================================================================================================
fn modify_foo1(mut foo: Box<i32>) -> i32 {
    *foo += 1;
    *foo
}
fn modify_foo2(foo: &mut i32) -> i32 {
    *foo += 1;
    *foo
}

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
    #[trace]
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
    #[trace]
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

pub fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    b
}
