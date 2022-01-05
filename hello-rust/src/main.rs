extern crate ferris_says;

use ferris_says::say;
use std::io::{stdout, BufWriter};

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

    println!("===============================================================");
    for number in (1..4).rev() {
        // [1, 4)
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    println!("===============================================================");
    println!("===============================================================");
    println!("===============================================================");
    println!("===============================================================");
    println!("===============================================================");
}

fn plus_one(x: i32) -> i32 {
    // return x + 1;
    x + 1
}
