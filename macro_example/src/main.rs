// https://www.youtube.com/watch?v=q6paRBbLgNw
//
fn main() {
    println!("x = {}", f());

    let mut x = 10;
    println!("x = {:?}", add_one!(x));
    println!("x = {:?}", x);
    let x: Vec<u32> = avec![];
    println!("avec = {:?}", x);
    let x: Vec<u32> = avec![32];
    println!("avec = {:?}", x);
    // let foo: String;
    // let bar: String;
    // let baz: String;
    let x: Vec<u32> = avec!(33, 34, 35; _foo, _bar, _baz);
    println!("avec = {:?}", x);
    // println!("foo = {:?}, bar = {}, baz = {}", foo, bar, baz);
    let x: Vec<u32> = bvec![32];
    println!("bvec = {:?}", x);
    let x: Vec<u32> = bvec![32, 33, 34];
    println!("bvec = {:?}", x);
    let s: Vec<&str> = bvec![
        "abdsfasdfasdgjasdgasdkfj;alksjdfuwqierjsjfl;kasjdf;ljk",
        "abdsfasdfasdgjasdgasdkfj;alksjdfuwqierjsjfl;kasjdf;ljk",
        "abdsfasdfasdgjasdgasdkfj;alksjdfuwqierjsjfl;kasjdf;ljk",
        "abdsfasdfasdgjasdgasdkfj;alksjdfuwqierjsjfl;kasjdf;ljk",
        "abdsfasdfasdgjasdgasdkfj;alksjdfuwqierjsjfl;kasjdf;ljk",
    ];
    println!("bvec = {:?}", s);
    println!("bvec = {:?}", bvec![12;3]);
    let mut x = Some(4);
    println!("bvec = {:?}", bvec![x.take().unwrap(); 3]);

    println!("============================designators==============================");
    foo();
    bar();

    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });

    println!("============================overload==============================");
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);

    println!("============================repeat==============================");
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));

    println!("============================DRY==============================");
    // Test `add_assign`, `mul_assign`, and `sub_assign`.
    // test_op!(add_assign, 1u32, 2u32, 3u32);
    // test_op!(mul_assign, 2u32, 3u32, 6u32);
    // test_op!(sub_assign, 3u32, 2u32, 1u32);

    println!("============================modules==============================");
    biz::music::music_test();

    println!("============================lazy_static==============================");
    // First access to `HASHMAP` initializes it
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

    // Any further access to `HASHMAP` just returns the computed value
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());

    println!("============================Variadics==============================");
    calculate! { // Look ma! Variadic `calculate!`!
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }

    println!("============================DRY==============================");
    println!("============================DRY==============================");
}

// ================================================lazy_static============================================
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

// ================================================Variadics============================================
#[macro_export]
macro_rules! calculate {
    // The pattern for a single `eval`
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    // Decompose multiple `eval`s recursively
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

// ================================================example============================================
fn f() -> i32 {
    let x = 1;
    let _v = vec![1, 2, 3];

    macro_rules! first_x {
        () => {
            x
        };
    }

    let x = 2;

    x + first_x!()
}

#[macro_export]
macro_rules! add_one {
    ($x: ident) => {{
        $x += 1;
        $x
    }};
}

#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };

    ($element: expr) => {{
        let mut vs = Vec::new();
        vs.push($element);
        vs
    }};
    ($($element: expr),+; $($x: ident),+) => {{ // 逗号分隔
        let mut vs = Vec::new();
        $(let $x = vs.push($element);)+
        // $(vs.push($element);)*
        // $(vs.push($x);)*
        vs
    }};
}

#[macro_export]
macro_rules! bvec {
    () => {
        Vec::new()
    };

    ($($element: expr),+ $(,)?) => {{ // 逗号分隔，最多只有一个逗号
        let mut vs = Vec::new();
        $(vs.push($element);)*
        vs
    }};

    ($element: expr; $count: expr) => {{
        // let mut vs = Vec::new();
        // let x = $element; // vs.push(x.take().unwrap()) will panic;
        // for _ in 0..$count {
        //     vs.push(x.clone());
        // }
        let count = $count;
        let mut vs = Vec::with_capacity(count);
        vs.extend(std::iter::repeat($element).take(count));
        vs
    }};
}

// ==============================================designators==============================================
macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// Create functions named `foo` and `bar` with the above macro.
create_function!(foo);
create_function!(bar);

#[macro_export]
macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

// ==============================================overload==============================================
// `test!` will compare `$left` and `$right`
// in different ways depending on how you invoke it:
#[macro_export]
macro_rules! test {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

// ================================================repeat============================================
// `find_min!` will calculate the minimum of any number of arguments.
#[macro_export]
macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}

// ================================================DRY(Don't repeat yourself)============================================
#[macro_export]
use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // The `tt` (token tree) designator is used for
    // operators and tokens.
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    };
}

// Implement `add_assign`, `mul_assign`, and `sub_assign` functions.
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

// mod test_op {
// use std::iter;
// #[macro_export]
// macro_rules! test_op {
//     ($func:ident, $x:expr, $y:expr, $z:expr) => {
//         // #[test]
//         fn $func() {
//             for size in 0usize..10 {
//                 let mut x: Vec<_> = iter::repeat($x).take(size).collect();
//                 let y: Vec<_> = iter::repeat($y).take(size).collect();
//                 let z: Vec<_> = iter::repeat($z).take(size).collect();

//                 // $func(&mut x, &y);
//                 super::$func(&mut x, &y);

//                 assert_eq!(x, z);
//             }
//         }
//     };
// }
// // Test `add_assign`, `mul_assign`, and `sub_assign`.
// test_op!(add_assign, 1u32, 2u32, 3u32);
// test_op!(mul_assign, 2u32, 3u32, 6u32);
// test_op!(sub_assign, 3u32, 2u32, 1u32);
// }

// ================================================modules============================================
mod schema {
    pub fn schema_test() {
        println!("Success!");
    }
}

mod biz {
    pub mod music {
        use super::super::schema::*;

        pub fn music_test() {
            schema_test();
        }
    }
}
