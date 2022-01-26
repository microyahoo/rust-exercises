// https://levelup.gitconnected.com/demystifying-closures-futures-and-async-await-in-rust-part-2-futures-abe95ab332a2
use std::convert::Infallible;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use futures::future;
use futures::future::FutureExt;
use log::debug;
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
use tokio::time::delay_for;

// https://docs.rs/tokio/0.2.20/tokio/attr.main.html
#[tokio::main(core_threads = 4)]
async fn main() -> Result<(), Box<dyn Error>> {
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
    // let mut rt = tokio::runtime::Builder::new()
    //     // .enable_all() // https://docs.rs/tokio/0.2.25/tokio/runtime/struct.Builder.html#method.enable_all
    //     .enable_time()
    //     .threaded_scheduler()
    //     .core_threads(4)
    //     .on_thread_start(|| debug!("on_thread_start()"))
    //     .build()
    //     .unwrap();

    // rt.enter(|| {
    // println!("in rt.enter()");
    // tokio::spawn(future::lazy(|_| println!("in tokio::spawn")));
    debug!("in rt.enter()");
    tokio::spawn(future::lazy(|_| debug!("in tokio::spawn")));
    // });

    // rt.spawn(future::lazy(|_| debug!("in rt::spawn")));
    tokio::spawn(future::lazy(|_| debug!("in rt::spawn")));
    // rt.block_on(future::lazy(|_| debug!("in rt.block_on()")));
    future::lazy(|_| debug!("in rt.block_on()")).await;
    // rt.spawn(future::lazy(|_| println!("in rt::spawn")));
    // rt.block_on(future::lazy(|_| println!("in rt.block_on()")));

    {
        // let result = rt.block_on(future::ready("Hello from rt.block_on()"));
        let result = future::ready("Hello from rt.block_on()").await;
        debug!("{}", result);
    }

    // the trait `std::marker::Unpin` is not implemented for `dyn futures::Future<Output = i32>`
    // rt.block_on(returns_dyn_future_i32());
    // rt.block_on(returns_pin_dyn_future_i32());
    returns_pin_dyn_future_i32().await;
    println!("=============================================================================");
    returns_future_chain().await;

    println!("=============================================================================");
    // let result = rt.enter(|| returns_delayed_future());
    // let result = rt.block_on(returns_delayed_future()); //thread 'main' panicked at 'there is no timer running, must be called from the context of a Tokio 0.2.x runtime'
    // debug!("{}", result);

    println!("=============================================================================");
    // todo!()

    // future::ready(42);
    // https://docs.rs/futures/latest/futures/future/trait.FutureExt.html
    // future::ready(42).boxed();

    println!("===================================async==========================================");
    async {
        async_hello().await;
    };
    // rt.block_on(async_hello());
    async_hello().await;
    {
        let x = 42;
        let async_capture = async {
            debug!("in async_capture, x => {}", x);
        };
        // rt.block_on(async_capture);
        async_capture.await;
    }
    {
        let x = 42;
        let async_capture = future::lazy(|_| {
            debug!("in async_capture, x => {}", x);
        });
        // rt.block_on(async_capture);
        async_capture.await;
    }
    // let r1 = rt.block_on(async_return_i32());
    let r1 = async_return_i32().await;
    debug!("async_return_i32 = {}", r1);
    let r2 = return_async_block_i32().await;
    debug!("return_async_block_i32 = {}", r2);

    println!(
        "===================================async/await=========================================="
    );
    // rt.block_on(async {
    async {
        debug!("in rt.block_on()");
        let r0 = future::ready("Hello from rt.block_on()").await;
        debug!("{}", r0);
        let r1 = returns_impl_future_i32().await;
        debug!("returns_impl_future_i32() -> {}", r1);
        // let r2 = returns_dyn_future_i32().await;
        // debug!("returns_dyn_future_i32() -> {}", r2);
        let r3 = returns_future_result().await;
        debug!("returns_future_result() -> {}", r3.unwrap());
        let r4 = returns_future_result_dyn_error().await;
        debug!("returns_future_result_dyn_error() -> {}", r4.unwrap());
        let r5 = returns_delayed_future().await;
        debug!("returns_delayed_future() -> {}", r5);
        let r6 = wait_a_sec(future::ready(42)).await;
        debug!("wait_a_sec(future::ready(42)) -> {}", r6);
        async_hello().await;
        let async_block = async {
            debug!("in async_block");
        };
        async_block.await;
        let x = 42;
        let async_capture = async {
            debug!("in async_capture, x => {}", x);
        };
        async_capture.await;
        let r7 = async_return_i32().await;
        debug!("async_return_i32 -> {}", r7);
        let r8 = returns_pin_dyn_future_i32().await;
        debug!("returns_pin_dyn_future_i32 -> {}", r8);
        let r9 = return_async_block_i32().await;
        debug!("return_async_block_i32 -> {}", r9);
        // });
    }
    .await;

    println!("===================================error==========================================");
    let _ = fallible().await?; // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
    Ok(())
}

async fn fallible() -> Result<(), Box<dyn Error>> {
    let _f = std::fs::File::open("foo.txt")?;
    Ok(())
}

async fn async_hello() {
    debug!("hello, asynchronously!");
}

async fn async_return_i32() -> i32 {
    42
}

fn return_async_block_i32() -> impl Future<Output = i32> {
    async { 43 }
}

fn returns_future_result_dyn_error() -> impl Future<Output = Result<i32, Box<dyn Error>>> {
    future::ok(42)
}

// NOTE: to return `impl Trait`, all returned values must be of the same type
fn returns_impl_future_i32() -> impl Future<Output = i32> {
    future::ready(42)
    // if rand::random() {
    //     return future::ready(42);
    // }
    // future::lazy(|_| 1337)
}

// NOTE: the trait `std::marker::Unpin` is not implemented for `dyn futures::Future<Output = i32>`
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

fn returns_future_chain() -> impl Future<Output = ()> {
    future::lazy(|_| debug!("in returns_future_chain()"))
        .then(|_| {
            debug!("in first then");
            future::ready("Hello from rt.block_on()")
        })
        .inspect(|result| debug!("future::ready() -> {}", result))
        .then(|_| returns_impl_future_i32())
        .inspect(|result| debug!("returns_impl_future_i32() -> {}", result))
        .then(|_| returns_pin_dyn_future_i32())
        .inspect(|result| debug!("returns_pin_dyn_future_i32() -> {}", result))
        .then(|_| returns_future_result())
        .map(|result| result.unwrap())
        .inspect(|result| debug!("returns_future_result().unwrap() -> {}", result))
        .then(|_| returns_delayed_future())
        .inspect(|result| debug!("returns_delayed_future() -> {}", result))
        .then(|_| wait_a_sec(future::ready(42)))
        .inspect(|result| debug!("wait_a_sec(future::ready(42)) -> {}", result))
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

fn wait_a_sec<F, O>(f: F) -> impl Future<Output = O>
where
    F: Future<Output = O>,
{
    let delay = Duration::from_millis(1000);
    delay_for(delay).then(|_| f)
}

fn returns_delayed_future() -> impl Future<Output = i32> {
    delay_for(Duration::from_millis(500)).then(|_| futures::future::ready(42))
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
