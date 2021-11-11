// ***********
// enum basics
// ***********

// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         println!("{:?}", self);
//     }
// }

// fn main() {
//     let v4 = IpAddr::V4(128, 0, 0, 21);
//     let v6 = IpAddr::V6(String::from("::1"));

//     println!("v4: {:#?}", v4);
//     println!("v6: {:#?}", v6);

//     let q = Message::Quit;
//     let m = Message::Move { x: 32, y: 32 };
//     let w = Message::Write(String::from(""));
//     let c = Message::ChangeColor(255, 255, 255);

//     println!("{:?}", q);
//     println!("{:?}", m);
//     println!("{:?}", w);
//     println!("{:?}", c);
//     q.call();
//     m.call();
//     w.call();
//     c.call();
// }

// // *************************************
// // enum with match, coin-sorting machine
// // *************************************

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky Penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main() {
//     let p = Coin::Penny;
//     println!("{}", value_in_cents(p));
// }

// // **************************************
// // enum with match, coin-sorting machine2
// // **************************************
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     Arizona,
//     Arkansas,
//     California,
//     Colorado,
//     Connecticut,
//     Delaware,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// impl Coin {
//     fn value_in_cents(&self) -> u8 {
//         match &self {
//             Coin::Penny => {
//                 println!("Lucky Penny!");
//                 1
//             }
//             Coin::Nickel => 5,
//             Coin::Dime => 10,
//             Coin::Quarter(state) => {
//                 println!("State quarter from {:?}!", state);
//                 25
//             }
//         }
//     }
// }

// fn main() {
//     let p = Coin::Penny;
//     println!("{}", p.value_in_cents());
//     let q = Coin::Quarter(UsState::Arkansas);
//     println!("{}", q.value_in_cents());
// }

// // ***********************
// // matching with Option<T>
// // ***********************
// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => {
//                 println!("{}", i + 1);
//                 Some(i + 1)
//             }
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five);
// }

// // ***********************************************************
// // match can be used on types other than enums (can use other)
// // ***********************************************************
// use rand::Rng;

// fn main() {
//     let dice_roll = rand::thread_rng().gen_range(1..10);
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other), // compiler warns if other is not used
//     }

//     fn add_fancy_hat() {
//         println!("Fancy hat awarded!");
//     }
//     fn remove_fancy_hat() {
//         println!("You lost a fancy hat.");
//     }
//     fn move_player(num_spaces: u8) {
//         println!("move {}", num_spaces);
//     }
// }

// ************************************************************************************
// match can be used on types other than enums (can use _ which compiler does not warn)
// ************************************************************************************

use rand::Rng;

fn main() {
    for _ in 1..10 {
        let dice_roll = rand::thread_rng().gen_range(1..10);
        println!("{}", dice_roll);
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (),
        }
    }

    fn add_fancy_hat() {
        println!("Fancy hat awarded!");
    }
    fn remove_fancy_hat() {
        println!("You lost a fancy hat.");
    }
}
