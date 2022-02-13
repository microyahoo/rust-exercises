use std::collections::HashMap;

fn main() {
    iter_demo();
    into_iter_demo();
    iter_mut_demo();

    println!("===================================================================================");
    // Vec example
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // Array example
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` for arrays unusually yields `i32`.
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));

    println!("===================================================================================");
    let mut x = HashMap::new();
    x.insert("1", 1);
    x.insert("2", 2);
    x.insert("3", 3);
    let y: Vec<(&str, usize)> = x.into_iter().collect();

    println!("===================================================================================");
}

// https://users.rust-lang.org/t/why-does-iter-and-into-iter-does-the-same-thing-if-the-object-is-a-reference/43848
fn foo(x: &HashMap<&str, usize>) {
    // This doesn't works
    // let y: Vec<(&str, usize)> = x.into_iter().collect();

    // This works
    let y: Vec<(&&str, &usize)> = x.into_iter().collect();
}

// https://gist.github.com/philipjkim/98bc460d31773255fd2b20780a73b742
fn iter_demo() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    // iter() returns an iterator of slices.
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

fn into_iter_demo() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.into_iter();

    // into_iter() returns an iterator from a value.
    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
}

fn iter_mut_demo() {
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();

    // iter_mut() returns an iterator that allows modifying each value.
    assert_eq!(v1_iter.next(), Some(&mut 1));
    assert_eq!(v1_iter.next(), Some(&mut 2));
    assert_eq!(v1_iter.next(), Some(&mut 3));
    assert_eq!(v1_iter.next(), None);
}
