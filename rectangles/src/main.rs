// // *******************
// // simple code
// // *******************
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels",
//         area(width1, height1)
//     )
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// // *******************
// // refactor with tuple
// // *******************
// fn main() {
//     let rect1 = (30, 50);

//     println!("The area of the rectangle is {} square pixels", area(rect1))
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// // ****************
// // refactor with structs
// // ****************
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println! {
//         "The area of the rectangle is {} square pixels", area(&rect1)
//     };
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// // **********************
// // debugging structs with println!
// // **********************
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {:#?}", rect1);
// }

// // with :?
// // rect1 is Rectangle { width: 30, height: 50 }

// // with :#?
// // rect1 is Rectangle {
// //     width: 30,
// //     height: 50,
// // }

// // ***************************
// // debugging structs with dbg!
// // ***************************
// #[derive(Debug)] // remains on --release
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale), // 1st dbg!
//         height: 50,
//     };

//     dbg!(&rect1); // 2nd dbg!
// }

// // 1st dbg!
// // [src/main.rs:90] 30 * scale = 60

// // 2nd dbg!
// // [src/main.rs:94] &rect1 = Rectangle {
// //     width: 60,
// //     height: 50,
// // }

// // ****************
// // defining methods
// // ****************
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn width(&self) -> bool {
//         // can use the same name as a field
//         self.width > 0
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );

//     if rect1.width() {
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }
// }

// // ****************************************************
// // practice: write a method that takes another instance
// // ****************************************************
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         let cond1 = self.width >= other.width && self.height >= other.height;
//         let cond2 = self.width >= other.height && self.height >= other.width;
//         cond1 || cond2
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };
//     let rect4 = Rectangle {
//         width: 45,
//         height: 25,
//     };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
//     println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));
// }

// *****************************************************************
// Associated Functions which do not need self, thus are not methods
// *****************************************************************
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle::square(3);
    println!("rect1: {:#?}", rect1);
}
