fn main() {
    // #################### The match Control Flow Operator ####################
    // Rust has an extremely powerful control flow operator called match that 
    // allows us to compare a value against a series of patterns and then execute
    // code based on which pattern matches. Patterns can be made up of literal values,
    // variable names, wildcards, and many other things.
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    let cents = value_in_cents(Coin::Nickel);
    println!("{}", cents);

    // First, we list the match keyword followed by an expression, which in this case 
    // is the value coin. Next are the match arms. An arm has two parts: a pattern and
    // some code. The first arm here has a pattern that is the value Coin::Penny and 
    // then the => operator that separates the pattern and the code to run. The code in
    // this case is just the value 1. Each arm is separated from the next with a comma.

    // When the match expression executes, it compares the resulting value against the 
    // pattern of each arm, in order. If a pattern matches the value, the code associated 
    // with that pattern is executed. If that pattern doesn’t match the value, execution 
    // continues to the next arm.

    // ************************ Patterns that Bind to Values ************************
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    enum Coin2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents2(coin: Coin2) -> u8 {
        match coin {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
    let cents = value_in_cents2(Coin2::Quarter(UsState::Alaska));
    println!("{}", cents);

    // ************************** Matching with Option<T> **************************
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:#?}, {:#?}, {:#?}", five, six, none);

    // ************************** Matches Are Exhaustive **************************
    // There’s one other aspect of match we need to discuss. Consider this version 
    // of our plus_one function that has a bug and won’t compile:
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i + 1),
    //     }
    // }
    // We didn’t handle the None case, so this code will cause a bug. Luckily, it’s 
    // a bug Rust knows how to catch. If we try to compile this code, we’ll get 
    // this error: pattern `None` not covered
    // Matches in Rust are exhaustive: we must exhaust every last possibility in 
    // order for the code to be valid.

    // *************************** The '_' Placeholder ***************************
    // Rust also has a pattern we can use when we don’t want to list all possible 
    // values. For example, a u8 can have valid values of 0 through 255. If we only
    // care about the values 1, 3, 5, and 7, we don’t want to have to list out 0, 2,
    //  4, 6, 8, 9 all the way up to 255.
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    // The _ pattern will match any value. By putting it after our other arms, the _ 
    // will match all the possible cases that aren’t specified before it.
}
