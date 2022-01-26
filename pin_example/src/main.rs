use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String, // 改成指针
                      // _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            // _marker: PhantomPinned, // This makes our type `!Unpin`
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        unsafe { &*(self.b) }
    }
}

#[derive(Debug)]
struct TestPin1 {
    a: String,
    b: *const String, // 改成指针
    _marker: PhantomPinned,
}

impl TestPin1 {
    fn new(txt: &str) -> Self {
        TestPin1 {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // This makes our type `!Unpin`
        }
    }

    fn init<'a>(self: Pin<&'a mut Self>) {
        let self_ref: *const String = &self.a;
        // self.b = self_ref;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ref;
    }

    fn a<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().a
    }

    fn b<'a>(self: Pin<&'a Self>) -> &'a String {
        unsafe { &*(self.b) }
    }
}

#[derive(Debug)]
struct TestPin2 {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl TestPin2 {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = TestPin2 {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // This makes our type `!Unpin`
        };
        let mut boxed = Box::pin(t);
        let self_ptr = &boxed.as_ref().a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };
        boxed
    }

    // fn init<'a>(self: Pin<&'a mut Self>) {
    //     let self_ref: *const String = &self.a;
    //     // self.b = self_ref;
    //     let this = unsafe { self.get_unchecked_mut() };
    //     this.b = self_ref;
    // }

    fn a<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().a
    }

    fn b<'a>(self: Pin<&'a Self>) -> &'a String {
        unsafe { &*(self.b) }
    }
}

fn main() {
    let mut x = String::from("xxx");
    let mut y = String::from("yyy");

    std::mem::swap(&mut x, &mut y);

    assert_eq!("yyy", &x);
    assert_eq!("xxx", &y);

    // let mut a = String::from("hello");
    // let _test = Test { a, b: &a };

    println!(
        "===================================================================================="
    );

    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("test1: a: {}, b: {}", test1.a(), test1.b());
    println!("test2: a: {}, b: {}", test2.a(), test2.b());

    // 使用swap()函数交换两者，这里发生了move
    std::mem::swap(&mut test1, &mut test2);
    println!("after swap, test1: a: {}, b: {}", test1.a(), test1.b());
    println!("after swap, test2: a: {}, b: {}", test2.a(), test2.b());

    test1.a = "I've totally changed now!".to_string();
    println!("after update, test1: a: {}, b: {}", test1.a(), test1.b());
    println!("after update, test2: a: {}, b: {}", test2.a(), test2.b());

    println!(
        "===================================================================================="
    );
    let mut t1 = TestPin1::new("test1");
    let mut t2 = TestPin1::new("test2");
    let mut test1 = unsafe { Pin::new_unchecked(&mut t1) };
    // test1.init(); // ERROR: borrow of moved value: `test1`
    TestPin1::init(test1.as_mut());
    let mut test2 = unsafe { Pin::new_unchecked(&mut t2) };
    // test2.init(); // borrow of moved value: `test1`
    TestPin1::init(test2.as_mut());

    println!(
        "test1: a: {}, b: {}",
        TestPin1::a(test1.as_ref()),
        TestPin1::b(test1.as_ref())
    );
    println!(
        "test2: a: {}, b: {}",
        TestPin1::a(test2.as_ref()),
        TestPin1::b(test2.as_ref())
    );

    std::mem::swap(&mut test1, &mut test2);
    // std::mem::swap(test1.get_mut(), test2.get_mut()); // `std::marker::PhantomPinned` cannot be unpinned
    println!(
        "after swap, test1: a: {}, b: {}",
        TestPin1::a(test1.as_ref()),
        TestPin1::b(test1.as_ref())
    );
    println!(
        "after swap, test2: a: {}, b: {}",
        TestPin1::a(test2.as_ref()),
        TestPin1::b(test2.as_ref())
    );
    // println!("after swap, test1: a: {}, b: {}", test1.a(), test1.b());
    // println!("after swap, test2: a: {}, b: {}", test2.a(), test2.b());

    // t1.a = "I've totally changed now!".to_string();
    // println!(
    //     "test1: a: {}, b: {}",
    //     TestPin1::a(test1.as_ref()),
    //     TestPin1::b(test1.as_ref())
    // );
    // println!(
    //     "test2: a: {}, b: {}",
    //     TestPin1::a(test2.as_ref()),
    //     TestPin1::b(test2.as_ref())
    // );

    println!(
        "===================================================================================="
    );
    let mut test1 = TestPin2::new("test1");
    let mut test2 = TestPin2::new("test2");

    println!(
        "test1: a: {}, b: {}",
        TestPin2::a(test1.as_ref()),
        TestPin2::b(test1.as_ref())
    );
    println!(
        "test2: a: {}, b: {}",
        TestPin2::a(test2.as_ref()),
        TestPin2::b(test2.as_ref())
    );

    std::mem::swap(&mut test1, &mut test2);
    // std::mem::swap(test1.get_mut(), test2.get_mut());
    println!(
        "after swap, test1: a: {}, b: {}",
        TestPin2::a(test1.as_ref()),
        TestPin2::b(test1.as_ref())
    );
    println!(
        "after swap, test2: a: {}, b: {}",
        TestPin2::a(test2.as_ref()),
        TestPin2::b(test2.as_ref())
    );
}
