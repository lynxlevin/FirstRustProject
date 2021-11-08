// // example for ownership and borrowing
// fn main() {
//     let s1 = gives_ownership();

//     let s2 = String::from("hello");
//     let s2 = takes_and_gives_back(s2);

//     println!("{}", s1);
//     println!("{}", s2);
//     // println!("{}", s3);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// // mutable references could only be borrowed once at a time
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2)
// }

// // this works, instead
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     println!("{}", r1);

//     let r2 = &mut s;
//     println!("{}", r2)
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point

//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }

fn main() {
    let word = String::from("hello, world");
    let hello = first_word(&word);
    println!("{}", hello);
    println!("{}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}