fn main() {
    // #################################### ENUMS ####################################
    // Enums allows us to define a type by enumerating its possible values. We can 
    // express this concept in code by defining an IpAddrKind enumeration and listing 
    // the possible kinds an IP address can be, V4 and V6. These are known as the 
    // variants of the enum:
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }
    // IpAddrKind is now a custom data type that we can use elsewhere in our code.

    // ******************************** Enum Values ********************************
    // We can create instances of each of the two variants of IpAddrKind like this:
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Enum Values: four = {:?}, six = {:#?}", four, six); // prints "V4, V6"

    // Note that the variants of the enum are namespaced under its identifier, and we
    // use a double colon to separate the two. The reason this is useful is that now 
    // both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind.
    // We can then, for instance, define a function that takes any IpAddrKind:
    fn route(ip_kind: IpAddrKind) { }
    // And we can call this function with either variant:
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // ------------  Example#1: use enum in IP address types with struct ------------
    #[derive(Debug)]
    enum IpAddrKind2 {
        V4,
        V6,
    }
    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind2,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind2::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind2::V6,
        address: String::from("::1"),
    };
    println!("Example#1: home = {:#?}, loopback = {:#?}", home, loopback);

    // ----------- Example#2: use enum in IP address types without struct -----------
    #[derive(Debug)]
    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));
    println!("Example#2: home = {:#?}, loopback = {:#?}", home, loopback);

    // Above code illustrates that we can put any kind of data inside an enum variant:
    // strings, numeric types, or structs, for example. we can even include another enum!

    // ------------------------------- Methods in Enum -------------------------------
    // As we’re able to define methods on structs using impl, we’re also able to 
    // define methods on enums.
    impl IpAddr2{
        fn call(&self) {
            println!("Methods in Enum: {:#?}", self);
        }
    }
    home.call();

    // ************* The Option Enum and Its Advantages Over Null Values *************
    // The Option type is used in many places because it encodes the very common 
    // scenario in which a value could be something or it could be nothing.
    // Rust doesn’t have the null feature that many other languages have. Null is a 
    // value that means there is no value there. In languages with null, variables can
    // always be in one of two states: null or not-null.
    // The problem with null values is that if you try to use a null value as a not-null
    // value, you’ll get an error of some kind. Because this null or not-null property 
    // is pervasive, it’s extremely easy to make this kind of error.

    // The problem isn’t really with the concept but with the particular implementation.
    // As such, Rust does not have nulls, but it does have an enum that can encode the 
    // concept of a value being present or absent. This enum is Option<T>, and it is 
    // defined by the standard library as follows:
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // The Option<T> enum is so useful that it’s even included in the prelude; we 
    // don’t need to bring it into scope explicitly. In addition, so are its variants:
    // we can use Some and None directly without the Option:: prefix.
    // <T> means the Some variant of the Option enum can hold one piece of data of any type.
    let some_number = Some(5);
    let some_string = Some("a string");
    // If we use None rather than Some, we need to tell Rust what type of Option<T> we 
    // have, because the compiler can’t infer the type that the Some variant will hold
    // by looking only at a None value.
    let absent_number: Option<i32> = None;

    // In short, because Option<T> and T (where T can be any type) are different types,
    // the compiler won’t let us use an Option<T> value as if it were definitely a 
    // valid value. the following code will not compile:
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; compile time error: no implementation for `i8 + std::option::Option<i8>`
    // Rust doesn’t understand how to add an i8 and an Option<i8>, because they’re different types.
    // We have to convert an Option<T> to a T before we can perform T operations with it.
}
