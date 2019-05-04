fn main() {
    // ################## OWNERSHIP ##################
    // *********** The Stack and the Heap ***********
    // The stack stores values in the order it gets them and removes 
    // the values in the opposite order. This is referred to as last in, first out.
    // All data stored on the stack must have a known, fixed size.
    // Data with unknown size at compile time or a size that might change 
    // stored on the heap. The heap is less organized: when you put 
    // data on the heap, you ask for some amount of space. The operating system finds 
    // an empty spot somewhere in the heap that is big enough, marks it as being in use, 
    // and returns a pointer, which is the address of that location.
    let x = 32; // stored in stack
    let s = String::from("Hello"); // stored in heap

    // ************** Ownership Rules **************
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // ************** Variable Scopes **************
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        // do stuff with s
    }   // this scope is now over, and s is no longer valid

    // ************** The String Type **************
    // Rust has a second string type, String. This type is allocated on the heap 
    // and as such is able to store an amount of text that is unknown to us at compile time. 
    // You can create a String from a string literal using the from function, like so:
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    // *********** Memory and Allocation ***********
    // With the String type, in order to support a mutable, growable piece of text, we need to 
    // allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
    // 1. The memory must be requested from the operating system at runtime.
    // 2. We need a way of returning this memory to the operating system when we’re done with our String.
    // That first part is done by us: when we call String::from
    // In Rust, the memory is automatically returned once the variable that owns it goes out of scope.
    {
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    }   // this scope is now over, and s is no longer valid
    //  When a variable goes out of scope, Rust calls a special function for us. This function 
    // is called drop, and it’s where the author of String can put the code to return the memory. 
    // Rust calls drop automatically at the closing curly bracket.
    
    // -------- Ways Variables and Data Interact: Move --------
    // Multiple variables can interact with the same data in different ways in Rust
    let mut x = 5;
    let y = x; // here x and y both have same value, because there size is fixed, these two push
    // on stack. here, y has a copy of x's value.
    let s1 = String::from("hello"); // here s1 is stored in heap. 
    let s2 = s1; // here, not a copy of s1 assigned to s2 but s1 is now moved with ownership
    // into s2 and after this s1 is no longer available, it is completely destroyed.
    // println!("{}", s1); here compile time error occurs with message: "value borrowed here after move"
    // the conclusion is that, in assigning a one variable to another: 
    // 1. if variable has fixed size stored in stack memory, copy of values takes place
    // 2. if variable has dynamic size stored in heap memory, movement with ownership takes place

    // -------- Ways Variables and Data Interact: Clone --------
    // If we do want to deeply copy the heap data, not just the stack data, we can use a 
    // common method called clone.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // ************** Ownership and Functions **************
    // Passing a variable to a function will move or copy, just as assignment does.
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s); // s's value moves into the function and so is no longer valid here
    // println!("{}", s); here compile time error with message: "value borrowed here after move"
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy
    println!("{}",x); // x exists here after passed into func

    // ************** Return Values and Scope **************
    // Returning values can also transfer ownership.
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also
    // moves its return value into s3
    // to get moved variable into func back, we can return moved variable from func 
    // in tuple like below code:
    let s1 = String::from("hello");
    let (s1, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.
fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
fn gives_ownership() -> String { // gives_ownership will move its return value into the function
    // that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}