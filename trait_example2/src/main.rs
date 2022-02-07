use std::io::Write;

trait Animal {
    fn talk(&self);
}

struct Cat {}

struct Dog {}

impl Animal for Cat {
    fn talk(&self) {
        println!("meow");
    }
}

impl Animal for Dog {
    fn talk(&self) {
        println!("bark");
    }
}

// `dyn` tells the compiler to not determine the exact type and just be content with a reference
// to some type implementing the trait `Animal`. This is called `dynamic dispatch` and the type
// is determined at the runtime, so there is a runtime overhead.
//
// Rust calls this a trait object (& dyn Animal). It represents a pointer to the concrete type
// and a pointer to a vtable of function pointers. Box<dyn Animal>, Rc<dyn Animal> are also
// trait Objects. They too contain a pointer to a concrete type allocated on the heap, that
// satisfies the given trait.
fn animal_talk(a: &dyn Animal) {
    a.talk();
}

// // FIXME: error
// fn animal_talk2(a: dyn Animal) {
//     a.talk();
// }

// `impl` here makes the compiler determine the type at the compile time, which means the compiler
// will do some `name mangling` and will have two variants of the functions; One that takes Dog
// and another that takes Cat. This is called `monomorphization` and will not have any runtime
// overhead, but will lead to code bloat.
fn animal_talk3(a: impl Animal) {
    a.talk();
}

fn is_dog_available() -> bool {
    true
}

fn animal() -> Box<dyn Animal> {
    if is_dog_available() {
        return Box::new(Dog {});
    }

    Box::new(Cat {})
}

// // FIXME: error
// fn animal2() -> impl Animal {
//     if is_dog_available() {
//         return Dog {};
//     }
//     Cat {}
// }

fn main() {
    let d = Dog {};
    let c = Cat {};
    animal_talk(&d);
    animal_talk(&c);

    let mut buf: Vec<u8> = vec![];
    // let writer: Write = buf;
    let writer: &mut dyn Write = &mut buf;
}

// https://cotigao.medium.com/dyn-impl-and-trait-objects-rust-fd7280521bea
