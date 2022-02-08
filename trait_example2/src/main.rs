use std::io::Write;

// ================================================================================================
// https://cotigao.medium.com/dyn-impl-and-trait-objects-rust-fd7280521bea
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

// ================================================================================================
// https://stackoverflow.com/questions/27567849/what-makes-something-a-trait-object
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

trait Print {
    fn print(&self);
}

// dyn Print is actually a type and we can implement methods on it
impl dyn Print + 'static {
    fn print_traitobject(&self) {
        println!("from trait object");
    }
}

impl Print for Point {
    fn print(&self) {
        println!("x: {}, y: {}, z: {}", self.x, self.y, self.z);
    }
}

// static dispatch (compile time): compiler must know specific versions
// at compile time generates a version for each type

// compiler will use monomorphization to create different versions of the function
// for each type. However, because they can be inlined, it generally has a faster runtime
// compared to dynamic dispatch
fn static_dispatch<T: Print>(point: &T) {
    point.print();
}

// dynamic dispatch (run time): compiler doesn't need to know specific versions
// at compile time because it will use a pointer to the data and the vtable.
// The vtable contains pointers to all the different different function implementations.
// Because it has to do lookups at runtime it is generally slower compared to static dispatch

// point_trait_obj is a trait object
fn dynamic_dispatch(point_trait_obj: &(dyn Print + 'static)) {
    point_trait_obj.print();
    point_trait_obj.print_traitobject();
}

fn main() {
    let d = Dog {};
    let c = Cat {};
    animal_talk(&d);
    animal_talk(&c);

    let mut buf: Vec<u8> = vec![];
    // let writer: Write = buf;
    let writer: &mut dyn Write = &mut buf;

    println!("================================================================");

    let point = Point { x: 1, y: 2, z: 3 };

    // On the next line the compiler knows that the generic type T is Point
    static_dispatch(&point);

    // This function takes any obj which implements Print trait
    // We could, at runtime, change the specfic type as long as it implements the Print trait
    dynamic_dispatch(&point);
}
