fn main(){
    // ########################################## Traits ##########################################
    // A trait tells the Rust compiler that a particular type has a functionality and it can share
    // this functionality with other types. We can use traits to define shared behavior in an 
    // abstract way. We can use trait bounds to specify that a generic can be any type that has 
    // certain behavior.

    // ************************************* Defining a Trait *************************************
    // A type’s behavior consists of the methods we can call on that type. Different types share the 
    // same behavior if we can call the same methods on all of those types. Trait definitions are a 
    // way to group method signatures together to define a set of behaviors necessary to accomplish 
    // some purpose. Defining a trait:
    pub trait Summary { // Here, we declare a trait using the trait keyword and then the trait’s name
        fn summarize(&self) -> String; // Inside the curly brackets, we declare the method signatures
        // that describe the behaviors of the types that implement this trait
    }
    // A trait can have multiple methods in its body: the method signatures are listed one per line
    // and each line ends in a semicolon.

    // ****************************** Implementing a Trait on a Type ******************************
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    // Now, we can implement the above defined trait "Summary" to struct "Tweet" like this:
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
            // fill in the method body with the specific behavior that we want the methods of the 
            // trait to have for the particular type.
        }
    }
    // Implementing a trait on a type is similar to implementing regular methods. The difference is
    // only in first line of impl, where after 'impl' trait name comes and after 'for' struct name.
    // After implementing the trait, we can call the methods on instances of Tweet in the 
    // same way we call regular methods, like this:
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    // One restriction to note with trait implementations is that we can implement a trait on a 
    // type only if either the trait or the type is local to our crate.
    // This restriction is part of a property of programs called coherence, and more specifically 
    // the orphan rule, so named because the parent type is not present. This rule ensures that 
    // other people’s code can’t break your code and vice versa.

    // ******************************** Default Implementations ********************************
    // Sometimes it’s useful to have default behavior for some or all of the methods in a trait 
    // instead of requiring implementations for all methods on every type.
    pub trait Summary2 {
        fn summarize2(&self) -> String {
            String::from("(Read more...)") // default behaviour
        }
    }
    // To use a default implementation to summarize instances of Tweet instead of defining 
    // a custom implementation, we specify an empty impl block with:
    impl Summary2 for Tweet {}
    println!("1 new tweet: {}", tweet.summarize2());

    // Default implementations can call other methods in the same trait, even if those other 
    // methods don’t have a default implementation.
    pub trait Summary3 {
        fn summarize_author(&self) -> String;
        fn summarize3(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    // To use this version of Summary, we only need to define summarize_author when we implement
    // the trait on a type:
    impl Summary3 for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }
    println!("1 new tweet: {}", tweet.summarize3());

    // ********************************* Traits as Parameters *********************************
    pub fn notify(item: impl Summary3) {
        println!("Breaking news! {}", item.summarize3());
    }
    // notify(tweet);

    // ---------------------------------- Trait Bound Syntax ----------------------------------
    // The impl Trait syntax works for straightforward cases, but is syntax sugar for a longer 
    // form. The longer syntax is called a trait bound, and it looks like this:
    pub fn notify2<T: Summary3>(item: T) {
        println!("Breaking news! {}", item.summarize3());
    }
    notify2(tweet);
    
    // to have two parameters that implement Summary, the impl Trait syntax would look like this:

    // pub fn notify(item1: impl Summary, item2: impl Summary) {

    // Defining this function using impl Trait would be appropriate if item1 and item2 were allowed
    // to have different types (as long as both types implement Summary). If you wanted to force 
    // both parameters to have the exact same type, that is only possible to express with a trait bound:

    // pub fn notify<T: Summary>(item1: T, item2: T) {

    // The generic type T specified as the type of the item1 and item2 parameters constrains the 
    // function such that the concrete type of the value passed as an argument for item1 and item2 
    // must be the same.

    // ------------------- Specifying Multiple Trait Bounds with the + Syntax -------------------
    // If notify needed to use display formatting on item as well as the summarize method, then 
    // the notify definition specifies that item must implement two traits: Display and Summary. 
    // This can be done using the + syntax:
    // pub fn notify(item: impl Summary + Display)
    // pub fn notify<T: Summary + Display>(item: T)

    // ------------------------- Clearer Trait Bounds with where Clauses -------------------------
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    // we can use a where clause, like this:
    // fn some_function<T, U>(t: T, u: U) -> i32
    //     where T: Display + Clone,
    //         U: Clone + Debug
    // {
    
    // ************************** Returning Types that Implement Traits **************************
    // We can use the impl Trait syntax in return position as well, to return a value of some type 
    // that implements a trait:
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
    let tweet2 = returns_summarizable();
    // The impl Summary return type means that the returns_summarizable function returns some type 
    // that implements the Summary trait, but doesn't specify the concrete type. In this case, 
    // returns_summarizable returns a Tweet, but the code calling this function doesn’t know that.
}