#![allow(dead_code)]

use std::cell::Cell;
use std::collections::HashSet;
use std::fmt;

fn lifetime_shortener<'a>(s: &'static str) -> &'a str {
    s
}

// fn cell_shortener<'a, 'b>(s: &'a Cell<&'static str>) -> &'a Cell<&'b str> {
//     s
// }

// fn lifetime_lengthener<'a>(s: &'a str) -> &'static str {
//     s
// }

// fn cell_lengthener<'a, 'b>(s: &'a Cell<&'b str>) -> &'a Cell<&'static str> {
//     s
// }

fn fn_ptr_lengthener<'a>(f: fn(&'a str) -> ()) -> fn(&'static str) -> () {
    f
}

// fn cell_example() {
//     // Consider this Cell. It holds a static string.
//     let foo: Cell<&'static str> = Cell::new("foo");

//     // Do you think this can work?
//     let owned_string: String = "non_static".to_owned();
//     foo.replace(&owned_string);

//     // Doesn't seem like it can, right? foo promises that what's inside it is
//     // a &'static str, but we tried to put in an owned string scoped to this
//     // function.
// }

// fn cell_counterexample() {
//     let foo: Cell<&'static str> = Cell::new("foo");
//     let owned_string: String = "non_static".to_owned();

//     // If we pretend that cell_shortener works
//     let shorter_foo = cell_shortener(&foo);

//     // then `shorter_foo` and `foo` would be aliases of each other, which would
//     // mean that you could use `shorter_foo` to replace `foo`s `Cell` with a
//     // non-static string:
//     shorter_foo.replace(&owned_string);

//     // Now `foo`, which is an alias of `shorter_foo`, has a non-static string
//     // in it! Whoops.
// }

struct OutlivesExample<'a, 'b: 'a> {
    a_str: &'a str,
    b_str: &'b str,
}

struct Multi<'a, 'b, 'c, 'd1, 'd2> {
    a: &'a str,
    b: Cell<&'b str>,
    c: fn(&'c str) -> usize,
    d: &'d1 mut &'d2 str,
}

fn a<'a, 'b, 'c, 'd1, 'd2>(x: Multi<'static, 'b, 'c, 'd1, 'd2>) -> Multi<'a, 'b, 'c, 'd1, 'd2> {
    x
}

fn c<'a, 'b, 'c, 'd1, 'd2>(x: Multi<'a, 'b, 'c, 'd1, 'd2>) -> Multi<'a, 'b, 'static, 'd1, 'd2> {
    x
}

fn d1<'a, 'b, 'c, 'd1, 'd2>(x: Multi<'a, 'b, 'c, 'static, 'd2>) -> Multi<'a, 'b, 'c, 'd1, 'd2> {
    x
}

struct TwoSpots<'a> {
    foo: &'a str,
    bar: Cell<&'a str>,
}

struct TypeParams<T, U> {
    t: Vec<T>,
    u: fn(U) -> (),
}

struct LifetimeParams<'a, 'b> {
    nested: TypeParams<&'a str, &'b str>,
}

fn lifetime_check<'a, 'b>(x: LifetimeParams<'static, 'b>) -> LifetimeParams<'a, 'static> {
    x
}

// Consider this struct representing a message.
struct Message<'msg> {
    message: &'msg str,
}

// ... this struct that collects messages to be displayed.
struct MessageCollector<'a, 'msg> {
    list: &'a mut Vec<Message<'msg>>,
}

impl<'a, 'msg> MessageCollector<'a, 'msg> {
    // This adds a message to the end of the list.
    fn add_message(&mut self, message: Message<'msg>) {
        self.list.push(message);
    }
}

// And this struct that displays collected messages.
struct MessageDisplayer<'a, 'msg> {
    list: &'a Vec<Message<'msg>>,
}

impl<'a, 'msg> fmt::Display for MessageDisplayer<'a, 'msg> {
    // This displays all the messages, separated by newlines.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for message in self.list {
            write!(f, "{}\n", message.message)?;
        }
        Ok(())
    }
}

fn message_example() {
    // Here's a simple pool of messages.
    let mut message_pool: HashSet<String> = HashSet::new();
    message_pool.insert("ten".to_owned());
    message_pool.insert("twenty".to_owned());

    // All right, let's try collecting and displaying some messages!
    collect_and_display(&message_pool);
}

fn collect_and_display<'msg>(message_pool: &'msg HashSet<String>) {
    let mut list = vec![];

    // Collect some messages. (This is pretty simple but you can imagine the
    // collector being passed into other code.)
    let mut collector = MessageCollector { list: &mut list };
    for message in message_pool {
        collector.add_message(Message { message });
    }

    // Now let's display those messages!
    let displayer = MessageDisplayer { list: &list };
    println!("{}", displayer);
}

struct SimpleMessageDisplayer<'a> {
    list: &'a Vec<Message<'a>>,
}

impl<'a> fmt::Display for SimpleMessageDisplayer<'a> {
    // This displays all the messages.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for message in self.list {
            write!(f, "{}\n", message.message)?;
        }
        Ok(())
    }
}

fn collect_and_display_2<'msg>(message_pool: &'msg HashSet<String>) {
    // OK, let's do the same thing as collect_and_display, except using the
    // simple displayer.
    let mut list = vec![];

    // Collect some messages.
    let mut collector = MessageCollector { list: &mut list };
    for message in message_pool {
        collector.add_message(Message { message });
    }

    // Finally, display them.
    let displayer = SimpleMessageDisplayer { list: &list };
    println!("{}", displayer);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        message_example();
    }
}
