// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for item in list.iter() {
//         if *item > largest {
//             largest = *item;
//         }
//     }

//     largest
// }

use advanced::aggregator;
use advanced::aggregator::Summary;

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

fn main() {
    // let number_list = vec![34, 56, 12, 9];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    println!("===============================================================");
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
    println!("1 new tweet: {:?}", tweet.sum());
    println!("1 new tweet: {:?}", tweet.summarize()); // NOTE: 需要 use advanced::aggregator::Summary;

    println!("=============================panic==================================");
    println!("=============================panic==================================");
    println!("=============================panic==================================");
}
