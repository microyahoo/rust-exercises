// https://levelup.gitconnected.com/demystifying-closures-futures-and-async-await-in-rust-part-2-futures-abe95ab332a2
//
use std::convert::Infallible;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

use futures::future;
use log::debug;
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};

fn main() {
    let config = ConfigBuilder::new()
        .set_target_level(LevelFilter::Trace)
        .build();
    let _ = SimpleLogger::init(LevelFilter::Debug, config);

    let add = |x, y| x + y;
    println!("{}", add(1, 2));
    let closure = curry(add, 5);
    println!("closure(1) => {}", closure(1));
    receives_closure(add);

    let two = 2;
    let add = |x, y| x + y + two;
    let closure = generic_curry(add, 4);
    receives_closure_one(closure);

    let concat = |s, t: &str| format!("{}{}", s, t);
    let closure = generic_curry(concat, "Hello, ");
    let result = closure("world!");
    println!("{}", result);

    let value = "hello".to_string().as_mut_str();

    {
        let y = 2;
        receives_closure_one(|x| x + y);
    }
    {
        let y = 3;
        receives_closure_one(|x| x + y);
    }

    let closure = returns_closure();
    receives_closure_one(closure);

    println!("=============================================================================");
    // let mut rt = tokio::runtime::Runtime::new().unwrap();
    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .core_threads(4)
        .on_thread_start(|| debug!("on_thread_start()"))
        .build()
        .unwrap();
    rt.enter(|| {
        // println!("in rt.enter()");
        // tokio::spawn(future::lazy(|_| println!("in tokio::spawn")));
        debug!("in rt.enter()");
        tokio::spawn(future::lazy(|_| debug!("in tokio::spawn")));
    });
    rt.spawn(future::lazy(|_| debug!("in rt::spawn")));
    rt.block_on(future::lazy(|_| debug!("in rt.block_on()")));
    // rt.spawn(future::lazy(|_| println!("in rt::spawn")));
    // rt.block_on(future::lazy(|_| println!("in rt.block_on()")));

    {
        let result = rt.block_on(future::ready("Hello from rt.block_on()"));
        debug!("{}", result);
    }

    // the trait `std::marker::Unpin` is not implemented for `dyn futures::Future<Output = i32>`
    // rt.block_on(returns_dyn_future_i32());
    rt.block_on(returns_pin_dyn_future_i32());

    // future::ready(42);
    // future::ready(42).boxed();
}

// NOTE: to return `impl Trait`, all returned values must be of the same type
fn returns_impl_future_i32() -> impl Future<Output = i32> {
    future::ready(42)
    // if rand::random() {
    //     return future::ready(42);
    // }
    // future::lazy(|_| 1337)
}

fn returns_dyn_future_i32() -> Box<dyn Future<Output = i32>> {
    if rand::random() {
        Box::new(future::ready(42))
    } else {
        Box::new(future::lazy(|_| 1337))
    }
}

fn returns_pin_dyn_future_i32() -> Pin<Box<dyn Future<Output = i32>>> {
    if rand::random() {
        Box::pin(future::ready(42))
    } else {
        Box::pin(future::lazy(|_| 1337))
    }
}

// fn returns_future_chain() -> impl Future<Output = ()> {
//     future::lazy(|_| debug!("in returns_future_chain()"))
//         .then(|_| {
//             debug!("in first then");
//             future::ready("Hello from rt.block_on()")
//         })
//         .inspect(|result| debug!("future::ready() -> {}", result))
//         .then(|_| returns_impl_future_i32())
//         .inspect(|result| debug!("returns_impl_future_i32() -> {}", result))
//         .then(|_| returns_dyn_future_i32())
//         .inspect(|result| debug!("returns_dyn_future_i32() -> {}", result))
//         .then(|_| returns_future_result())
//         .map(|result| result.unwrap())
//         .inspect(|result| debug!("returns_future_result().unwrap() -> {}", result))
//         .then(|_| returns_future_result_dyn_error())
//         .map(|result| result.unwrap())
//         .inspect(|result| debug!("returns_future_result_dyn_error().unwrap() -> {}", result))
//         .then(|_| returns_delayed_future())
//         .inspect(|result| debug!("returns_delayed_future() -> {}", result))
//         .then(|_| wait_a_sec(future::ready(42)))
//         .inspect(|result| debug!("wait_a_sec(future::ready(42)) -> {}", result))
//         .then(|_| {
//             debug!("in last then");
//             future::ready(())
//         })
// }

fn returns_future_chain() -> impl Future<Output = ()> {
    future::lazy(|_| debug!("in returns_future_chain()"))
        .then(|_| {
            debug!("in first then");
            future::ready("Hello from rt.block_on()")
        })
        .inspect(|result| debug!("future::ready() -> {}", result))
        .then(|_| returns_impl_future_i32())
        .inspect(|result| debug!("returns_impl_future_i32() -> {}", result))
        .then(|_| returns_dyn_future_i32())
        .inspect(|result| debug!("returns_dyn_future_i32() -> {}", result))
        .then(|_| returns_future_result())
        .map(|result| result.unwrap())
        .inspect(|result| debug!("returns_future_result().unwrap() -> {}", result))
        .then(|_| {
            debug!("in last then");
            future::ready(())
        })
}

fn returns_future_result() -> impl Future<Output = Result<i32, impl Error>> {
    // future::ok(32) // cannot resolve opaque type
    future::ok::<i32, Infallible>(42)
}

fn returns_future_result2() -> impl Future<Output = Result<i32, Box<dyn Error>>> {
    future::ok(32) // cannot resolve opaque type
                   // future::ok::<i32, Infallible>(42)
}

// fn receives_closure(closure: Fn(i32) -> i32) {} // Error
// fn receives_closure(closure: Box<dyn Fn(i32) -> i32>) {}
fn receives_closure<T>(closure: T)
where
    T: Fn(i32, i32) -> i32,
{
    let result = closure(1, 2);
    println!("closure(1, 2) => {}", result);
}

fn receives_closure_one<F>(closure: F)
where
    F: Fn(i32) -> i32,
{
    let result = closure(1);
    println!("closure(1) => {}", result);
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 4
}

fn curry<F>(f: F, x: i32) -> impl Fn(i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    move |y| f(x, y)
}

fn generic_curry<F, X, Y, Z>(f: F, x: X) -> impl Fn(Y) -> Z
where
    F: Fn(X, Y) -> Z,
    X: Copy,
{
    move |y| f(x, y)
}
