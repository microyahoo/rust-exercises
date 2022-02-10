use std::any::TypeId;
use std::io::Write;
use std::ptr::NonNull;
use std::{fmt, mem};

// ================================================================================================
// https://cotigao.medium.com/dyn-impl-and-trait-objects-rust-fd7280521bea
trait Animal {
    fn talk(&self);
}

struct Cat;

struct Dog;

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

// FIXME: error
// fn animal3() -> Animal {}

fn test<T: Animal>(arg: T) {
    arg.talk();
}

fn test2(arg: &dyn Animal) {
    arg.talk();
}

fn test3(arg: Box<dyn Animal>) {
    arg.talk();
}

// https://zhuanlan.zhihu.com/p/23791817
pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}

// 参数是 trait object 类型，p 是一个胖指针
fn print_traitobject(p: &dyn Animal) {
    // 使用 transmute 执行强制类型转换，把变量 p 的内部数据取出来
    let (data, vtable): (usize, usize) = unsafe { mem::transmute(p) };
    println!("TraitObject    [data:{}, vtable:{}]", data, vtable);
    unsafe {
        // 使用 as 执行强制类型转换，将 vtable 从 `usize` 类型转为 `*const usize` 类型
        let v: *const usize = vtable as *const () as *const usize;
        // 打印出指针 v 指向的内存区间的值
        println!(
            "data in vtable [{}, {}, {}, {}]",
            *v,
            *v.offset(1),
            *v.offset(2),
            *v.offset(3)
        );
    }
}

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

    println!("================================================================");
    let cat = Cat;
    let p_cat = &cat;
    let p_dog = p_cat as &dyn Animal;
    println!(
        "Size of p_cat {}, Size of p_dog {}",
        mem::size_of_val(&p_cat),
        mem::size_of_val(&p_dog)
    );

    let cat_talk: usize = unsafe { mem::transmute::<fn(&Cat), usize>(Cat::talk) };
    let dog_talk: usize = unsafe { mem::transmute::<fn(&Dog), usize>(Dog::talk) };
    println!("Cat::talk {}", cat_talk);
    println!("Dog::talk {}", dog_talk);

    print_traitobject(p_dog);
    let dog = Dog;
    print_traitobject(&dog as &dyn Animal);

    println!("================================================================");
    let x = 1_i32;
    x.foo1();
    x.foo2();
    let p = &x as &dyn Foo;
    p.foo1();
    // p.foo2(); // FIXME: error

    let mut i = 1;
    // let p: &mut dyn Double = &mut i as &mut dyn Double;
    // p.double();

    println!("================================================================");
    let a1 = Any::new(42_u32);
    let a2 = Any::new(String::from("hello"));
    dbg!(a1.type_id());
    dbg!(a2.type_id());

    dbg!(a1.downcast::<u32>().unwrap());
    dbg!(a2.downcast_ref::<String>().unwrap());

    let a3 = a2.clone();
    dbg!(a3.downcast_ref::<String>().unwrap());

    drop(a2);
    drop(a3);
}

// =================================================================================
// https://zhuanlan.zhihu.com/p/370713385
pub struct Any {
    data: NonNull<()>, // Box<T>
    vtable: &'static AnyVTable,
}

unsafe impl Send for Any {}
unsafe impl Sync for Any {}

struct AnyVTable {
    type_id: unsafe fn() -> TypeId,
    drop: unsafe fn(*mut ()),
    clone: unsafe fn(*const ()) -> Any,
}

impl AnyVTable {
    unsafe fn v_type_id<T>() -> TypeId
    where
        T: Send + Sync + 'static,
    {
        TypeId::of::<T>()
    }

    unsafe fn v_drop<T>(this: *mut ())
    where
        T: Send + Sync + 'static,
    {
        drop(Box::from_raw(this.cast::<T>()))
    }

    unsafe fn v_clone<T>(this: *const ()) -> Any
    where
        T: Clone + Send + Sync + 'static,
    {
        let x = Clone::clone(&*this.cast::<T>());
        Any::new(x)
    }
}

impl Any {
    pub fn new<T>(x: T) -> Self
    where
        T: Clone + Send + Sync + 'static,
    {
        unsafe {
            Self {
                data: NonNull::new_unchecked(Box::into_raw(Box::new(x))).cast(),
                vtable: &AnyVTable {
                    type_id: AnyVTable::v_type_id::<T>,
                    drop: AnyVTable::v_drop::<T>,
                    clone: AnyVTable::v_clone::<T>,
                },
            }
        }
    }

    pub fn type_id(&self) -> TypeId {
        unsafe { (self.vtable.type_id)() }
    }

    pub fn downcast<T>(self) -> Result<Box<T>, Self>
    where
        T: Send + Sync + 'static,
    {
        if self.type_id() == TypeId::of::<T>() {
            let ptr = self.data.as_ptr().cast::<T>();
            mem::forget(self);
            unsafe { Ok(Box::from_raw(ptr)) }
        } else {
            Err(self)
        }
    }

    pub fn downcast_ref<T>(&self) -> Result<&T, ()>
    where
        T: Send + Sync + 'static,
    {
        if self.type_id() == TypeId::of::<T>() {
            let ptr = self.data.as_ptr().cast::<T>();
            unsafe { Ok(&*ptr) }
        } else {
            Err(())
        }
    }
}

impl Clone for Any {
    fn clone(&self) -> Self {
        unsafe { (self.vtable.clone)(self.data.as_ptr()) }
    }
}

impl Drop for Any {
    fn drop(&mut self) {
        unsafe { (self.vtable.drop)(self.data.as_ptr()) }
    }
}

impl fmt::Debug for Any {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Any {{ .. }}")
    }
}

// =================================================================================
trait Foo {
    fn foo1(&self);
    fn foo2(&self)
    where
        Self: Sized;
}

impl Foo for i32 {
    fn foo1(&self) {
        println!("foo1 {}", self);
    }
    fn foo2(&self) {
        println!("foo2 {}", self);
    }
}

trait Double {
    fn new() -> Self;
    fn double(&mut self);
}

impl Double for i32 {
    fn new() -> i32 {
        0
    }
    fn double(&mut self) {
        *self *= 2;
    }
}
