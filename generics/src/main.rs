// // ****************************
// // an example with one instance
// // ****************************

// struct Point<T> { // declares a generic type T (could be named anything)
//     x: T, // uses the generic type T
//     y: T, // uses the same generic type T
// }

// impl<T> Point<T> { // declares T after imple to indicate that T in Point is generic
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> { // can use a specific type instead of generic T. in this case, unable to impl function x, because it conflicts on generic x.
//     fn y(&self) -> &f32 {
//         &self.y
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };
//     println!("p.x = {}", p.x());

//     let p_f32 = Point { x: 5.0, y: 10.0 };
//     println!("p_f32.y = {}", p_f32.y());
// }

// // *************************************************************
// // an example with two instances, which may have different types
// // *************************************************************

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> { // declares new generics V and W, which the other Point might have
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c' };

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

// // ************
// // using traits
// // ************

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };

//     println!("1 new tweet: {}", tweet.summarize());
// }

// // *****************************************
// // using traits with default implementations
// // *****************************************

// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)") // this is default implementation
//     }
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location) // this overrides
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {} // this is needed to declare Tweet implements Summary

// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };
//     println!("1 new tweet: {}", tweet.summarize()); // default impl

//     let news_article = NewsArticle {
//         headline: String::from("Breaking News!!"),
//         location: String::from("Burgundy"),
//         author: String::from("MY"),
//         content: String::from("Nothing special happened :b"),
//     };
//     println!("1 new article: {}", news_article.summarize()); // overridden impl
// }

// // ****************************
// // using traits like interfaces
// // ****************************

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// fn main() {
//     pub fn notify1(item: &impl Summary) {
//         println!("this is impl trait syntax {}", item.summarize());
//     }
//     pub fn notify2<T: Summary>(item: &T) {
//         println!("this is trait bound syntax {}", item.summarize());
//     }
//     pub fn notify3<T>(item: &T)
//     where
//         T: Summary,
//     {
//         println!("this is also trait bound syntax {}", item.summarize());
//     }

//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };

//     let news_article = NewsArticle {
//         headline: String::from("Breaking News!!"),
//         location: String::from("Burgundy"),
//         author: String::from("MY"),
//         content: String::from("Nothing special happened :b"),
//     };

//     notify1(&tweet);
//     notify2(&news_article);
//     notify3(&news_article);
// }

// // *************************
// // traits exercise (largest)
// // *************************

// // try compiling without PartialOrd or Copy
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// // this is another way without Copy trait. maybe one way to avoid heap location?
// // fn largest<T: PartialOrd>(list: &[T]) -> &T {
// //     let mut largest = &list[0];
// //     for item in list {
// //         if item > largest {
// //             largest = &item;
// //         }
// //     }
// //     largest
// // }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest(&number_list);
//     println!("{}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let result = largest(&char_list);
//     println!("{}", result);
// }

// // ******************************************************************
// // generic type parameters, trait bounds and lifetime in one function
// // ******************************************************************

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result =
//         longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
//     println!("The Longest string is {}", result);
// }

// use std::fmt::Display;

// fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
// where
//     T: Display,
// {
//     println!("Announcement! {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// *************
// trait objects
// *************

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // this is a trait object
}

// without trait objects, Screen would be like this, in which case,
// the components would need to be of the same one type T.
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("This is a Button.")
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("This is a SelectBox.")
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
