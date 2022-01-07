#![allow(unused)]
extern crate ferris_says;

use ferris_says::say;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::io::{stdout, BufWriter};

struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping HasDrop!");
    }
}

struct HasTwoDrops {
    one: HasDrop,
    two: HasDrop,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}

fn main() {
    println!("===============================================================");
    println!("Hello, world!");
    let out = b"Hello fellow Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();

    println!("===============================================================");
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // NOTE: 这里没有分号
    };
    println!("The value of x and y: {} {}", x, y);

    println!("===============================================================");
    let z = plus_one(x);
    println!("The value of z: {}", z);

    println!("===============================================================");
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    println!("===============================================================");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    println!("===============================================================");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // 如果将返回值加入你用来停止循环的 break 表达式，它会被停止的循环返回
        }
    };

    println!("The result is {}", result);

    println!("===============================================================");
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for (i, element) in a.iter().enumerate() {
        println!("the value of {} is: {}", i, element);
    }

    println!("===============================================================");
    for number in (1..4).rev() {
        // [1, 4)
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    println!("===============================================================");
    #[derive(Copy, Clone)]
    struct Foo(u8);

    let x = 1;
    let y = Foo(2);
    drop(x); // a copy of `x` is moved and dropped
    drop(y); // a copy of `y` is moved and dropped

    println!("x: {}, y: {}", x, y.0); // still available

    println!("===============================================================");
    // https://doc.rust-lang.org/std/mem/fn.drop.html
    let x = RefCell::new(1);

    let mut mutable_borrow = x.borrow_mut();
    *mutable_borrow = 1;

    drop(mutable_borrow); // relinquish the mutable borrow on this slot

    let borrow = x.borrow();
    println!("{}", *borrow);

    println!("===============================================================");
    // https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop
    {
        let _x = HasTwoDrops {
            one: HasDrop,
            two: HasDrop,
        };
        println!("Running!");
    }

    println!("===============================================================");
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // let s3 = s1; // borrow of moved value: `s1`

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    println!("===============================================================");
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);

    println!("===============================================================");
    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("the first world is: {}", word);

    let word = first_word(&s[..]);
    println!("the first world is: {}", word);

    let s = "hi world"; // s 的类型是 &str：它是一个指向二进制程序特定位置的 slice
    let word = first_word(s);
    println!("the first world is: {}", word);

    println!("===============================================================");
    let user1 = User {
        username: String::from("hi"),
        email: String::from("xxx@gmail.com"),
        active: true,
        sign_in_count: 2,
    };
    let user2 = User {
        username: String::from("hello"),
        email: String::from("yyy@gmail.com"),
        ..user1 // struct update syntax
    };

    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    dbg!(&user1);
    println!("{:#?}, {:#?}, {:#?}, {:#?}", user1, user2, black, origin);
    // println!("{:?}, {:?}, {:?}, {:?}", user1, user2, black, origin);

    println!("===============================================================");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("{:#?}", Rectangle::square(20));

    println!("===============================================================");
    let some_number = Some(5);
    let some_string = Some("string");
    let absent_number: Option<i32> = None;
    println!("absent_number.is_none(): {}", absent_number.is_none());
    println!("some_number.is_none(): {}", some_number.is_none());
    println!("some_number.is_some(): {}", some_number.is_some());

    let text: Option<String> = Some("Hello, world!".to_string());
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume *that* with `map`, leaving `text` on the stack.
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("still can print text: {:?}", text);

    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {}
    }
    println!("mut x: {:?}", x);

    println!("=======================match========================================");
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("Others");
    }

    println!("=============================vector==================================");
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("vector: {:?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    let second: i32 = v[1];
    println!("The second element is {}", second);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    v.pop();
    println!("vector: {:?}", v);
    for i in &v {
        println!("{}", i);
    }

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6); // cannot borrow `v` as mutable because it is also borrowed as immutable
    // println!("The first element is: {}", first);

    println!("=============================String==================================");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("after push_str, s1 is {}", s1);
    println!("s2 is {}", s2);
    s1.push('x');
    println!("after push, s1 is {}", s1);
    println!("===============================================================");
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("s = {}", s);

    println!("===============================================================");
    let s1 = String::from("hello");
    // let s2 = s1;
    // println!("s1 = {}", s1); // error[E0382]: borrow of moved value: `s1`
    //
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    println!("===============================================================");
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    println!("===============================================================");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("s3 = {}", s3);
    let s4 = s3 + " haha";
    println!("s4 = {}", s4);

    println!("===============================================================");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s = {}", s);

    println!("===============================================================");
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s = {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    println!("=============================HashMap==================================");
    let mut scores = HashMap::new();
    scores.insert(String::from("hello"), 1);
    scores.insert(String::from("hi"), 2);
    println!("scores = {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores = {:?}", scores);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score = {:?}", score);
    match score {
        Some(n) => println!("score = {}", n),
        None => println!("team not exists"),
    };

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("===============================================================");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    let e = scores.entry(String::from("Yellow"));
    println!("entry = {:#?}", e);
    e.or_insert(50);
    let e = scores.entry(String::from("Blue"));
    println!("entry = {:#?}", e);
    e.or_insert(50);
    println!("{:?}", scores);

    println!("===============================================================");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用。
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map = {:?}", map);

    println!("=============================result==================================");
    let f = File::open("hello.txt");
    println!("f = {:?}", f);
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem opening the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", error),
        },
    };

    println!("===============================================================");
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("===============================================================");
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    println!("===============================================================");
    // let mut f = File::open("hello.txt")?; // cannot use the `?` operator in a function that returns `()`

    println!("===============================================================");
    println!("=============================panic==================================");
    let v = vec![1, 2, 3];
    v[99];
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn plus_one(x: i32) -> i32 {
    // return x + 1;
    x + 1
}

// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
