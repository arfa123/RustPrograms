fn main() {
    // ######################################### STRUCTURE #########################################
    // A struct, or structure, is a custom data type that lets you name and package together multiple
    //  related values that make up a meaningful group. If you’re familiar with an object-oriented 
    // language, a struct is like an object’s data attributes.
    
    // **************************** Defining and Instantiating Structs ****************************
    // Structs are similar to tuples. Like tuples, the pieces of a struct can be different types. 
    // Unlike with tuples, you’ll name each piece of data so it’s clear what the values mean.
    // To define a struct, we enter the keyword struct and name the entire struct. Then, inside 
    // curly brackets, we define the names and types of the pieces of data, which we call fields.
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    // To use the struct after we’ve defined it, we create an instance of that struct by stating 
    // the name of the struct and then add curly brackets containing key: value pairs, where the 
    // keys are the names of the fields and the values are the data we want to store in those fields.
    let user1 = User {
        email: String::from("someone@example.com"), username: String::from("someusername123"),
        active: true, sign_in_count: 1,
    };
    // To get a specific value from a struct, we can use dot notation.
    println!("user email is: {}", user1.email);
    // If the instance is mutable, we can change a value by using the dot notation and assigning 
    // into a particular field.
    let mut user1 = User {
        email: String::from("someone@example.com"), username: String::from("someusername123"),
        active: true, sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com"); // Note that the entire instance must 
    // be mutable; Rust doesn’t allow us to mark only certain fields as mutable.

    // We can create a new instance of struct by using function like this:
    fn build_user(email: String, username: String) -> User { // the return type is struct name
        User {
            email: email, username: username, active: true, sign_in_count: 1,
        }
    }
    let user2 = build_user(String::from("abc@abc.com"), String::from("abc"));

    // ======== Using the Field Init Shorthand when Variables and Fields Have the Same Name ========
    // Because the parameter names and the struct field names are exactly the same, we can use 
    // the field init shorthand syntax
    fn build_user2(email: String, username: String) -> User {
        User {
            email, username, active: true, sign_in_count: 1,
        }
    }

    // ============= Creating Instances From Other Instances With Struct Update Syntax =============
    // we can use the one instance value in another instance like this:
    let user2 = User {
        email: String::from("another@example.com"), username: String::from("anotherusername567"),
        active: user1.active, sign_in_count: user1.sign_in_count,
    };
    let user2 = User {
        email: String::from("another@example.com"), username: String::from("anotherusername567"),
        ..user1 // The syntax .. specifies that the remaining fields not explicitly set should have 
        // the same value as the fields in the given instance.
    };

    // ============ Using Tuple Structs without Named Fields to Create Different Types ============
    // You can also define structs that look similar to tuples, called tuple structs. Tuple structs
    //  have the added meaning the struct name provides but don’t have names associated with their
    //  fields; rather, they just have the types of the fields.
    // To define a tuple struct, start with the struct keyword and the struct name followed by 
    // the types in the tuple.
    struct Color(i32, f32, u32);
    let black = Color(0, 9.3, 7);

    // =========================== Unit-Like Structs Without Any Fields ===========================
    // You can also define structs that don’t have any fields! These are called unit-like structs 
    // because they behave similarly to (), the unit type. Unit-like structs can be useful in 
    // situations in which you need to implement a trait on some type but don’t have any data that
    // you want to store in the type itself.

    // ***************************** An example program using Struct *****************************
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
    // ======================= How print struct instance in println! macro =======================
    // If want to print struct instance directly, we can't do this:
    // println!("rect1 is: {}", rect1); compile time error occurs: "main::Rectangle` cannot be 
    // formatted with the default formatter"
    // To print struct instance first add the annotation #[derive(Debug)] just before the struct 
    // definition and put the specifier :? between curly brackets {:?} 
    println!("rect1 is: {:?} {:#?}", rect1, rect1);
}