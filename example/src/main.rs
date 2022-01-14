#![feature(unboxed_closures)]
#![feature(fn_traits)]

extern crate num_bigint;
extern crate num_traits;
extern crate stacker;

use std::slice::Iter;
use std::vec::IntoIter;

use num_bigint::{BigUint, ToBigUint};
use num_traits::One;

use anyhow::Result;
use serde_yaml::Value;
use tokio::{fs, try_join};

// =============================================================================================
// https://users.rust-lang.org/t/how-can-i-use-generics-to-return-different-errors-from-a-function-using-anyhow/44221
#[macro_use]
use anyhow::Error;
use derive_more::Display; // naming it clearly for illustration purposes
use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};

impl Distribution<CustomError> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CustomError {
        match rng.gen_range(0, 4) {
            0 => CustomError::CustomOne,
            1 => CustomError::CustomTwo,
            2 => CustomError::CustomThree,
            _ => CustomError::CustomFour,
        }
    }
}

#[derive(Debug, Display)]
pub enum CustomError {
    #[display(fmt = "Custom Error 1")]
    CustomOne,
    #[display(fmt = "Custom Error 2")]
    CustomTwo,
    #[display(fmt = "Custom Error 3")]
    CustomThree,
    #[display(fmt = "Custom Error 4")]
    CustomFour,
}

impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // println!("{}", "hello");
        // self.source() // NOTE: [rustc unconditional_recursion] [W] function cannot return without recursing
        std::fmt::Error.source()
    }
}

#[derive(Debug)]
struct MyAnyhowError(Error);

impl std::fmt::Display for MyAnyhowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<MyAnyhowError> for CustomError {
    fn from(_item: MyAnyhowError) -> Self {
        rand::random::<CustomError>()
    }
}

impl From<CustomError> for MyAnyhowError {
    fn from(item: CustomError) -> Self {
        MyAnyhowError(Error::new(item))
    }
}

impl From<Error> for MyAnyhowError {
    fn from(item: Error) -> Self {
        MyAnyhowError(item)
    }
}

impl From<MyAnyhowError> for Error {
    fn from(item: MyAnyhowError) -> Self {
        item.0
    }
}

/// randomly returns either () or one of the 4 CustomError variants
fn do_something_random() -> Result<(), CustomError> {
    let mut rng = thread_rng();

    // 20% chance that () will be returned by this function
    if rng.gen_bool(2.0 / 10.0) {
        Ok(())
    } else {
        Err(rand::random::<CustomError>())
    }
}

fn do_something_anyhow() -> Result<(), Error> {
    Err(Error::msg("anyhow"))
}

fn do_something<E>() -> Result<(), E>
where
    // E: From<MyAnyhowError>, // compilation error
    E: From<CustomError> + From<anyhow::Error>,
{
    do_something_random()?; // <- First compilation error

    do_something_anyhow()?; // <- Second compilation error
    Ok(())
}

// =============================================================================================
#[tokio::main]
async fn main() -> Result<()> {
    // let xx = std::error::Error;

    println!("{}", "==========================1========================");
    // 读取 Cargo.toml，IO 操作 1
    let f1 = fs::read_to_string("./Cargo.toml");
    // 读取 Cargo.lock，IO 操作 2
    let f2 = fs::read_to_string("./Cargo.lock");
    let (content1, content2) = try_join!(f1, f2)?;

    // 计算
    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    // 写入 /tmp/Cargo.yml，IO 操作 3
    let f3 = fs::write("/tmp/Cargo.yml", &yaml1);
    // 写入 /tmp/Cargo.lock，IO 操作 4
    let f4 = fs::write("/tmp/Cargo.lock", &yaml2);
    try_join!(f3, f4)?;

    // 打印
    println!("{}", yaml1);
    println!("{}", yaml2);

    println!("{}", "==========================2========================");
    let num: u64 = 100;
    println!("Factorial {}! = {}", num, factorial(num));

    println!("{}", "==========================3========================");
    recurse(10);

    println!("{}", "==========================4========================");
    let x = Foo;
    x();

    println!("{}", "==========================5========================");
    let x = String::from("hello world");
    let consume_and_return_x = move || x;
    consume_with_relish(consume_and_return_x);

    println!("{}", "==========================6========================");
    let func: Functionish = make_func();
    func();

    println!("{}", "==========================7========================");
    let one_plus = Plus { n: 1 };
    let sum = one_plus(2);
    assert_eq!(sum, 1 + 2);
    dbg!(one_plus(2));

    let f = &Callable;
    assert_eq!(f(2), 1 + 2);

    println!("{}", "==========================8========================");
    let mut buffer: String = String::from("hello world");
    let slice = &buffer[1..];
    // buffer.push_str("hi"); // NOTE: dangling reference
    println!("slice = {}", slice);

    println!("{}", "==========================9========================");
    do_something::<anyhow::Error>().unwrap();
    Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(&content)?;
    Ok(serde_yaml::to_string(&value)?)
}

// =============================================================================================
// #[derive(thiserror::Error, Debug)]
// enum Error {
//     #[error("invalid token provided to internal function")]
//     InvalidToken,

//     #[error("underlying IO error")]
//     IoError(#[from] std::io::Error),

//     #[error("error generated by event source or callback")]
//     CallbackError(#[from] Box<dyn std::error::Error + Sync + Send>),
// }

// type Result<T> = core::result::Result<T, Error>;

// =============================================================================================
fn factorial(num: u64) -> BigUint {
    // println!("Called with: {}", num);
    let current: BigUint = num.to_biguint().unwrap();
    if num <= 1 {
        // println!("Returning...");
        return One::one();
    }

    // https://stackoverflow.com/questions/39840663/recursive-function-calculating-factorials-leads-to-stack-overflow
    stacker::maybe_grow(1024 * 1024, 1024 * 1024, || current * factorial(num - 1))
}

// =============================================================================================
use std::io::{Read, Write};
use std::net::TcpStream;
type IOError = std::io::Error;

pub trait Transport: Read + Write {
    /// Read up to buf.len() bytes from the underlying transport
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IOError>;
}

impl Transport for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IOError> {
        // return TcpStream::read(self, buf); // NOTE: [rustc E0034] [E] multiple applicable items in scope
        return <TcpStream as Read>::read(self, buf);
    }
}

// =============================================================================================
fn recurse(n: i32) {
    let v = match n % 2 {
        0 => n / 2,
        _ => 3 * n + 1,
    };
    println!("{}", v);

    if v != 1 {
        recurse(v)
    }
}

// =============================================================================================
fn consume_with_relish<F>(func: F)
where
    F: FnOnce() -> String, // TODO: ?
{
    // `func` consumes its captured variables, so it cannot be run more
    // than once.
    println!("Consumed: {}", func());

    println!("Delicious!");

    // Attempting to invoke `func()` again will throw a `use of moved
    // value` error for `func`.
}

// =============================================================================================
// https://stackoverflow.com/questions/42859330/how-do-i-make-a-struct-callable
struct Foo;

impl Fn<()> for Foo {
    extern "rust-call" fn call(&self, _args: ()) {
        println!("Call (Fn) for Foo");
    }
}

impl FnMut<()> for Foo {
    extern "rust-call" fn call_mut(&mut self, _args: ()) {
        println!("Call (FnMut) for Foo");
    }
}

impl FnOnce<()> for Foo {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: ()) {
        println!("Call (FnOnce) for Foo");
    }
}

// =============================================================================================
// https://github.com/dtolnay/case-studies/blob/master/callable-types/demo/main.rs
use std::mem::{self, MaybeUninit};
use std::ops::Deref;

/// Function object that adds some number to its input.
struct Plus {
    n: u32,
}

impl Plus {
    fn call(&self, arg: u32) -> u32 {
        self.n + arg
    }
}

impl Deref for Plus {
    type Target = dyn Fn(u32) -> u32;

    fn deref(&self) -> &Self::Target {
        let uninit_callable = MaybeUninit::<Self>::uninit();
        let uninit_closure = move |arg: u32| Self::call(unsafe { &*uninit_callable.as_ptr() }, arg);
        let size_of_closure = mem::size_of_val(&uninit_closure);
        fn second<'a, T>(_a: &T, b: &'a T) -> &'a T {
            b
        }
        let reference_to_closure = second(&uninit_closure, unsafe { mem::transmute(self) });
        mem::forget(uninit_closure);
        assert_eq!(size_of_closure, mem::size_of::<Self>());
        let reference_to_trait_object = reference_to_closure as &dyn Fn(u32) -> u32;
        reference_to_trait_object
    }
}

// =============================================================================================
struct Callable;

impl Deref for Callable {
    type Target = fn(u32) -> u32;

    fn deref(&self) -> &'static Self::Target {
        &(one_plus as fn(u32) -> u32)
    }
}

// impl Callable {
//     fn call(&self, arg: u32) -> u32 {
//         // Function body
//         1 + arg
//     }
// }

// impl Deref for Callable {
//     type Target = dyn Fn(u32) -> u32;

//     fn deref(&self) -> &Self::Target {
//         &|arg| self.call(arg) // NOTE: returning this value requires that `'1` must outlive `'static`
//     }
// }

fn get_owned_iterator() -> IntoIter<i32> {
    // fn get_dangling_iterator<'a>() -> Iter<'a, i32> {
    let v = vec![1, 2, 3];
    // v.iter()
    v.into_iter()
}

fn one_plus(arg: u32) -> u32 {
    1 + arg
}

// =============================================================================================
fn make_func() -> Functionish {
    Functionish {
        f: Box::new(|| println!("printing")),
    }
}

struct Functionish {
    f: Box<dyn Fn()>,
}

impl std::ops::Deref for Functionish {
    type Target = dyn Fn();

    fn deref(&self) -> &Self::Target {
        &self.f
    }
}

// =============================================================================================
pub fn calculate_iter(slice_a: &[i32], slice_b: &[i32]) -> i64 {
    slice_a
        .iter()
        .zip(slice_b.iter())
        .filter_map(|(a, b)| match *a > 2 {
            true => Some(*a as i64 * *b as i64),
            false => None,
        })
        .sum()
}

// =============================================================================================

// =============================================================================================

// =============================================================================================

// =============================================================================================

// =============================================================================================

// =============================================================================================
