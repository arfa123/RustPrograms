fn main() {
    // ************ IMMUTABLE & MUTABLE VARIABLES ***************
    // In Rust by default the declared variable is immutable.
    let a = 5;
    //a = 6; at this line, compiler gives error: "cannot assign twice to immutable variable"
    // to make it mutable, we need to add keyword "mut" after keyword "let"
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // ******************* CONSTANT VARIABLES ********************
    // constant variables are always immutable
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // ********************* SHADOWING **********************
    // you can declare a new variable with the same name as a previous variable,
    // and the new variable shadows the previous variable
    let y = 5;
    let y = y + 4;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // ****************** VARIABLES DATA TYPE CHANGING **********************
    // we can't change the data type of variable
    let mut b = "abc"; // this variable is type String
    //b = b.len(); at this line compiler gives error: "expected &str, found usize"
    //'b.len()' returns length of variable b of type number
    // Changing the variable can be possible in shadowing, below code works
    let c = "abc";
    println!("The value of before shadowing: {}", c);
    let c = c.len();
    println!("The value of c after shadowing: {}", c);
}
