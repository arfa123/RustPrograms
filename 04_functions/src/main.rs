fn main() {
    println!("Hello, world!");
    another_function(5, (6, -1), [2, 4], 'c', "abc");

    // ****** Function bodies contain statements and expressions ******
    // Function bodies are made up of a series of statements optionally ending in an expression
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resulting value.
    // let x = (let y = 6); at this line compiler gives error: "expected expression, found statement (`let`)"
    // The let y = 6 statement does not return a value, so there isn’t anything for x to bind to.
    let y = {
        let x = 3;
        x + 1
    }; // The block that we use to create new scopes, {}, is an expression
    println!("The value of y is: {}", y);
    // Expressions do not include ending semicolons. If you add a semicolon to the end of an expression,
    // you turn it into a statement, which will then not return a value
    // ***************** Functions with return values *****************
    // We don’t name return values, but we do declare their type after an arrow (->)
    // function’s return type is specified too, as -> i32
    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(3);
    println!("The value of x is: {}", x)
}
fn another_function(x: u8, tup: (u8, i8), arr: [u8; 2], c: char, s: &str) { // must declare the type of each parameter
    println!("Value of x is: {}", x);
    println!("Value of tup second index is: {}", tup.0);
    println!("Value of a's first index is: {}", arr[0]);
}
fn five() -> f32 {
    5.0
}
fn plus_one(x: i32) -> i32 {
    x + 1 //if we place a semicolon at the end of the line containing x + 1,
    // changing it from an expression to a statement, we’ll get an error: "expected i32, found ()"
}