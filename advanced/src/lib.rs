pub mod aggregator {

    use std::ops::Deref;

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    #[derive(Debug)]
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    impl Tweet {
        pub fn summ(&self) -> String {
            self.summarize()
        }
    }

    // impl Summary for Vec<T> {}

    // impl Deref for Tweet {
    //     type Target = Tweet;

    //     fn deref(&self) -> &Tweet {
    //         &self.0
    //     }
    // }
}
