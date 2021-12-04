#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests_for_rectangle {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    // this is for integration test
    a + b
}

#[cfg(test)]
mod tests_for_adder {
    use super::*;

    // assert_eq! and assert_ne! need their arguments to #derive[(PartialEq, Debug)];
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn input_does_not_equal_to_output() {
        assert_ne!(2, add_two(2));
    }
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests_for_greeting {
    use super::*;

    // on failing, this shows a message like below
    // thread 'tests_for_greeting::greeting_contains_name' panicked at 'Greeting did not contain name, value was Hello!', src/lib.rs:78:9
    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(name);
        assert!(
            result.contains(name),
            "Greeting did not contain name, value was {}",
            result
        );
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }
}

// assuring that a panic occurs
#[cfg(test)]
mod tests_for_guess {
    use super::*;

    // can specify a panic message, giving a part suffices(no need to give the whole message)
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic]
    fn smaller_than_1() {
        Guess::new(0);
    }
}

#[cfg(test)]
mod tests_for_result {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a); // in tests, this gets printed only on failure
                                       // cargo test -- --show-output lets it be printed even on success
    10
}

#[cfg(test)]
mod tests_for_prints_and_returns_10 {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }
}
