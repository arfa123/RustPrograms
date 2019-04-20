fn main() {
    // ############## IF Expressions ##############
    // *********** if else expressions ***********
    let number = 3;
    if number < 5 { // condition must be of type "bool"
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // ****** Handling multiple with else if ******
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // ************ Using if in a let ************
    // Because if is an expression, we can use it on the right side of a let statement
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    // ############ Repetition with loops ############
    // Rust has three kinds of loops: loop, while, and for.
    // *********** Repetition with loop ***********
    // The loop keyword tells Rust to execute a block of code over and over again
    // forever or until you explicitly tell it to stop.
    // when to stop executing the loop, you can place the break keyword within the loop
    loop {
        println!("again!");
        break;
    }
    // -------- Returning values from loop --------
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };
    println!("The result is {}", result);

    // ******** Conditional loops with while ********
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // ***** Looping through a collection with for *****
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
