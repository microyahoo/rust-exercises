use std::fmt::Debug;
use std::fmt::Display;

use advanced::aggregator;
use advanced::aggregator::Summary;
use std::env;
use std::sync::mpsc;
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
    let value = Rc::new(RefCell::new(5));
    // let a = Cons(Rc::clone(&value), Rc::new(Nil));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(16)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

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
            thread::sleep(Duration::from_secs(1));
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
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got {}", received);
    }
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
