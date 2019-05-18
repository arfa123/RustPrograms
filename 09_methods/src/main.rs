fn main() {
    // ############################## METHODS ##############################
    // Methods are similar to functions. However, methods are different
    // from functions in that they're defined within the context of a struct
    // and their first parameter is always self, which represents the instance 
    // of the struct the method is being called on.

    // ************************* Defining Methods *************************
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    // To define the function within the context of Rectangle, we start an 
    // impl (implementation) block.
    impl Rectangle{
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    // then create instance of struct Rectangle
    let rect1 = Rectangle { width: 30, height: 50 };
    // then call the method area of this instance like rect1.area()
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // ******************* Methods with More Parameters *******************
    impl Rectangle {
        fn area2(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // *********************** Associated Functions ***********************
    // Another useful feature of impl blocks is that we’re allowed to define
    // functions within impl blocks that don’t take self as a parameter. 
    // These are called associated functions because they’re associated with
    // the struct. They’re still functions, not methods, because they don’t
    // have an instance of the struct to work with. We’ve already used the
    // String::from associated function.
    // Associated functions are often used for constructors that will return 
    // a new instance of the struct.
    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle { width: size, height: size }
        }
    }
    // To call this associated function, we use the :: syntax with the struct name
    let sq = Rectangle::square(3);
    println!("{:#?}", sq);

    // *********************** Multiple impl Blocks ***********************
    // Each struct is allowed to have multiple impl blocks.
}