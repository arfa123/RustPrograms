fn main() {
    // ################ References and Borrowing ################
    // In Rust, we can use the variable reference and pass to any function
    // without transferring its ownership.
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // This ampersands are references, 
    // and they allow you to refer to some value without taking ownership of it
    println!("The length of '{}' is {}.", s1, len);
    // The &s1 syntax lets us create a reference that refers to the value of 
    // s1 but does not own it. Because it does not own it, the value it points
    // to will not be dropped when the reference goes out of scope.
    // We call having references as function parameters borrowing. As in real life, 
    // if a person owns something, you can borrow it from them.

    // Just as variables are immutable by default, so are references. We’re not 
    // allowed to modify something we have a reference to.
    let s = String::from("hello");
    change(&s);

    // ****************** Mutable References ******************
    // We can fix the above code error by below code:
    let mut s = String::from("hello"); // First, we had to change s to be mut.
    change2(&mut s); // Then we had to create a mutable reference with &mut s
    println!("The value of s is: {}", s);
    // But mutable references have one big restriction: you can have only one 
    // mutable reference to a particular piece of data in a particular scope. This code will fail:
    let r1 = &mut s;
    // let r2 = &mut s; // compile time error occurs: second mutable borrow occurs here
    // println!("{}, {}", r1, r2);
    // The benefit of having this restriction is that Rust can prevent data races at compile time.
    // A data race is similar to a race condition and happens when these three behaviors occur:
    // 1. Two or more pointers access the same data at the same time.
    // 2. At least one of the pointers is being used to write to the data.
    // 3. There’s no mechanism being used to synchronize access to the data.
    change2(&mut s);
    change2(&mut s); // this dual mutual refrencing works, because these both are in different scopes.
    println!("The value of s is: {}", s);

    // A similar rule exists for combining mutable and immutable references. We also cannot 
    // have a mutable reference while we have an immutable one.
    // This code results in an error:
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM compile time error occurs: mutable borrow occurs
    // println!("{}, {}, and {}", r1, r2, r3);

    // ***************** Dangling References *****************
    //  dangling pointer, a pointer that references a location in memory that may have been 
    // given to someone else, by freeing some memory while preserving a pointer to that memory.
    // In Rust, by contrast, the compiler guarantees that references will never be dangling references: 
    // if you have a reference to some data, the compiler will ensure that the data will not go out 
    // of scope before the reference to the data does.
    // Let’s try to create a dangling reference, which Rust will prevent with a compile-time error:
    // let reference_to_nothing = dangle();
    // fn dangle() -> &String { // // dangle returns a reference to a String
    //     let s = String::from("hello"); // s is a new String
    //     &s // we return a reference to the String, s
    // } // Here, s goes out of scope, and is dropped. Its memory goes away.
    // Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. 
    // But we tried to return a reference to it. That means this reference would be pointing to 
    // an invalid String. That’s no good! Rust won’t let us do this.

    // **************** The Rules of References ****************
    // Let’s recap what we’ve discussed about references:
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &String) {
    // some_string.push_str(", world"); compile time error occurs: `some_string` 
    // is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

fn change2(some_string: &mut String) { // accept a mutable reference with some_string: &mut String
    some_string.push_str(", world");
}
