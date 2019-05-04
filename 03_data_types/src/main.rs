fn main() {
    // Rust has a two data type subsets: Scalar and Compound
    // ################ 1. SCALAR TYPES ################
    // A scalar type represents a single value.
    // Rust has four primary scalar types: integers, floating-point, Booleans, and characters.
    // *************** a. INTEGER TYPES ***************
    // Integer has two types: Signed and Unsigned
    // Signed types start with 'i' and Unsigned types start with 'u'
    let x: i8 = -2; // signed type with length 8 bits
    let x: u8 = 2; // unsigned type with length 8 bits
    // possible lengths are 8, 16, 32, 64, 128 bits
    // Additionally, the 'isize' and 'usize' types depend on the kind of computer your program is running on
    // Integer types default to i32
    // ------------ Integer Overflow ------------
    let mut x: u8 = 255; // it can store max number 255
    // x = x + 1; // it overflows x, in debug mode program exit with panic, 
    // while in release mode, Rust will perform something called two’s complement wrapping.
    // In short, values greater than the maximum value the type can hold "wrap around" to 
    // the minimum of the values the type can hold.

    // ************ b. FLOATING POINT TYPES ************
    // floating-point types are f32 and f64, which are 32 bits and 64 bits in size.
    // The default type is f64
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("Value of x is: {} and y is: {}", x, y);

    // ----------- Numeric Operations -----------
    let sum = 5.7 + 10.5; // addition
    let difference = 9.0 - 4.3; // subtraction
    let product = 4.0 * 30.5; // multiplication
    let quotient = 56.0 / 32.2; // division
    let remainder = 43 % 5; // remainder
    println!("{}, {}, {}, {}, {}", sum, difference, product, quotient, remainder);

    // *************** c. BOOLEAN TYPES ***************
    // Boolean type in Rust has two possible values: true and false.
    // Booleans are one byte in size. The Boolean type in Rust is specified using bool
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("Value of t is: {} and f is: {}", t, f);

    // *************** d. CHARACTER TYPE ***************
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Value of c is: {}, z is: {} and heart_eyed_cat is: {}", c, z, heart_eyed_cat);

    // ################# 2. COMPOUND TYPES #################
    // Compound types can group multiple values into one type.
    // Rust has two primitive compound types: tuples and arrays.
    // ****************** a. TUPLE tYPE ******************
    // A tuple is a general way of grouping together some number of other values with a variety of types
    // into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // Each position in the tuple has a type
    let tup: (i32, f64, u8, char, &str) = (500, 6.4, 1, 'c', "abc");
    let (x, y, z, c, s) = tup; // destructure a tuple value
    let five_hundred = tup.0; // access a tuple element directly by using a period (.) followed by the index of the value we want to access
    let six_point_four = tup.1;
    println!("{}, {}, {}, {}, {}, {}, {}", x, y, z, c, s, five_hundred, six_point_four);

    // ****************** b. ARRAY TYPE ******************
    // Another way to have a collection of multiple values is with an array.
    // Unlike a tuple, every element of an array must have the same type.
    // Arrays in Rust have a fixed length, like tuples.
    let a = [1, 2, 3, 4, 5];
    // Writing an array's type is done with square brackets containing the type of each element
    // in the array followed by a semicolon and the number of elements in the array, like so:
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // a = [3, 3, 3, 3, 3];
    let first = a[0];
    let second = a[1];
    println!("{}, {}", first, second);
    // ---------- Invalid Array Element Access ----------
    // let index = 10;
    // let element = a[index];
    // The compilation didn’t produce any errors, but the program resulted in a 
    // runtime error and didn’t exit successfully
}
