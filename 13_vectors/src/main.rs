fn main() {
    // ######################################### VECTORS #########################################
    // Rust’s standard library includes a number of very useful data structures called collections.
    // Vectors allow us to store more than one value in a single data structure that puts all the
    // values next to each other in memory. Vectors can only store values of the same type.

    // ********************************** Creating a New Vector **********************************
    // To create a new, empty vector, we can call the Vec::new function:
    let v: Vec<i32> = Vec::new();
    // Note that we added a type annotation here. Because we aren’t inserting any values into this 
    // vector, Rust doesn’t know what kind of elements we intend to store.
    // Vectors are implemented using generics. When a specific vector holds a specific type, the 
    // type is specified within angle brackets. We’ve told Rust that the Vec<T> in v will hold 
    // elements of the i32 type.
    // We can also create a Vec<T> that has initial values, and Rust provides the vec! macro for 
    // convenience.
    let mut v = vec![1, 2, 3];
    // Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, and 
    // the type annotation isn’t necessary.

    // *********************************** Updating a Vector ***********************************
    // To create a vector and then add elements to it, we can use the push method:
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    // The numbers we place inside are all of type i32, and Rust infers this from the data, so we 
    // don’t need the Vec<i32> annotation.

    // ************************** Dropping a Vector Drops Its Elements **************************
    // Like any other struct, a vector is freed when it goes out of scope
    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here

    // ******************************* Reading Elements of Vectors *******************************
    // There are two ways to reference a value stored in a vector, with indexing syntax and the 
    // get method.
    let mut v = vec![1, 2, 3, 4, 5];
    // vectors are indexed by number, starting at zero.
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    // by using the get method with the index passed as an argument, which gives us an Option<&T>.
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // let’s see what a program will do if it has a vector that holds five elements and then tries 
    // to access an element at index 100
    // let does_not_exist = &v[100]; // this method will cause the program to panic because it references
    // a nonexistent element. This method is best used when you want your program to crash if there’s
    // an attempt to access an element past the end of the vector.

    let does_not_exist = v.get(100); // it returns None without panicking. You would use this method 
    // if accessing an element beyond the range of the vector happens occasionally under normal circumstances.

    // When the program has a valid reference, the borrow checker enforces the ownership and 
    // borrowing rules. Recall the rule that states you can’t have mutable and immutable references 
    // in the same scope.
    let first = &v[0];
    // v.push(6); // compile time error "mutable borrow occurs here" occurs
    println!("The first element is: {}", first);

    // ************************** Iterating over the Values in a Vector **************************
    // If we want to access each element in a vector in turn, we can iterate through all of the 
    // elements rather than use indices to access one at a time.
    for i in &v {
        println!("{}", i);
    }

    // We can also iterate over mutable references to each element in a mutable vector in order 
    // to make changes to all the elements.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("Vector v after changes: {:#?}", v);
    // To change the value that the mutable reference refers to, we have to use the dereference 
    // operator (*) to get to the value in i before we can use the += operator.

    // ************************** Using an Enum to Store Multiple Types **************************
    // At the beginning of this chapter, we said that vectors can only store values that are the 
    // same type. This can be inconvenient; there are definitely use cases for needing to store a 
    // list of items of different types. Fortunately, the variants of an enum are defined under 
    // the same enum type, so when we need to store elements of a different type in a vector, we 
    // can define and use an enum!
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Vector row with enum type: {:#?}", row);
}
