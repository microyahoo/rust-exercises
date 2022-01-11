use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Deref;

use advanced::aggregator;
use advanced::aggregator::Summary;
use std::env;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T, V> {
    x: T,
    y: V,
}

impl<T, V> Point<T, V> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &V {
        &self.y
    }

    fn mixup<U, W>(self, other: Point<U, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify1<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn some_function<T, U>(_t: T, _u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

// longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("=========================largest======================================");
    let number_list = vec![34, 56, 12, 9];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let string_list = vec![
        String::from("hello"),
        String::from("hi"),
        String::from("advanced"),
        String::from("ipad"),
    ];
    let result = largest(&string_list);
    println!("The largest string is {}", result);

    println!("============================Point===================================");
    let p = Point { x: 5, y: 6.06 };
    println!("p = {:?}", p);
    println!("p.x = {:?}", p.x());
    println!("p.y = {:?}", p.y());
    let p = Point { x: 5.78, y: 6.06 };
    println!("p.distance_from_origin = {:?}", p.distance_from_origin());
    println!("===============================================================");
    let p1 = Point { x: 5.78, y: 6.06 };
    let p2 = Point {
        x: "hello",
        y: "world",
    };
    let p3 = p1.mixup(p2);
    println!("p1.mixup(p2) = {:?}", p3);
    println!("=============================trait==================================");
    let tweet = aggregator::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {:?}", tweet);
    println!("1 new tweet: {:?}", tweet.summ());
    println!("1 new tweet: {:?}", tweet.summarize()); // NOTE: 需要 use advanced::aggregator::Summary;

    notify(tweet);

    println!("{}", 3.to_string());

    println!("=============================lifetime==================================");
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);

    let _case_insensitive = env::var("CASE_INSENSITIVE").is_err();

    println!("=============================Rc and RefCell==================================");
    let a = Rc::new(Cons1(5, Rc::new(Cons1(10, Rc::new(Nil1)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons1(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons1(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    println!("===============================================================");

    let value = Rc::new(RefCell::new(5));
    // let a = Cons(Rc::clone(&value), Rc::new(Nil));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(16)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10; // 自动解引用

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    println!("===============================================================");

    let mock_messager = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messager, 100);
    limit_tracker.set_value(80);
    println!(
        "message length: {}",
        mock_messager.sent_messages.borrow().len()
    );
    println!("===============================================================");

    let a = Rc::new(Cons2(5, RefCell::new(Rc::new(Nil2))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons2(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    println!("===============================================================");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("=============================closure==================================");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    println!("=============================Iterator==================================");
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    let total: i32 = v1_iter.sum(); // 调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权。
    println!("sum = {:?}", total);

    let v1 = vec![1, 2, 3];
    for i in v1.iter().map(|x| x + 1) {
        println!("i = {}", i);
    }
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2 = {:?}", v2);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);
    println!("{:?}", in_my_size);

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(2))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("sum = {}", sum);

    println!("=============================deref==================================");
    let x = 5;
    let y = MyBox::new(x);
    println!("mybox = {}", *y); // *(y.deref())

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    println!("=============================drop==================================");
    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }

    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        std::mem::drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
    println!("=============================channel==================================");
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("1 hi"),
            String::from("1 from"),
            String::from("1 the"),
            String::from("1 thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        // NOTE: move 不能少
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("Got {}", received);
    }

    println!("=============================Mutex and Arc==================================");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

// begin =============================macro==================================
#[macro_export]
macro_rules! vec_example {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
// end =============================macro==================================

// begin =============================drop==================================
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
// end =============================drop==================================

// begin =============================deref==================================
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// end =============================deref==================================

// begin =============================iterate==================================
#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect() // into_iter 来创建一个获取 vector 所有权的迭代器
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// begin =============================closure==================================
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
// end =============================closure==================================

// begin =============================Rc and RefCell==================================
use crate::List::{Cons, Nil};
use crate::List1::{Cons1, Nil1};
use crate::List2::{Cons2, Nil2};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List1 {
    Cons1(i32, Rc<List1>),
    Nil1,
}

#[derive(Debug)]
enum List2 {
    Cons2(i32, RefCell<Rc<List2>>),
    Nil2,
}

impl List2 {
    fn tail(&self) -> Option<&RefCell<Rc<List2>>> {
        match self {
            Cons2(_, item) => Some(item),
            Nil2 => None,
        }
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

// =======================================================================================
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// =======================================================================================
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

struct MockMessenger {
    // sent_messages: Vec<String>,
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
            // sent_messages: vec![],
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
        // self.sent_messages.push(String::from(message));
    }
}
// end =============================Rc and RefCell==================================

// fn live() {
//     let a: &usize;
//     {
//         let b: usize = 5;
//         a = &b;
//     }
//     println!("{}", a);
// }
