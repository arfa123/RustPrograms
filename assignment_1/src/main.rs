extern crate chrono;
use chrono::offset::{TimeZone, Utc};
use std::io;
use std::io::Write;
const PI: f32 = 3.14;

fn main() {
    q1();
    q2();
    q3();
    q4();
    q5();
    q6();
    q7();
    q8();
    q9();
    // q10();
    q11();
    q12();
    q13();
    q14();
    q15();
    q16();
    q17();
    q18();
    q19();
    q20();
    q21();
    q22();
    q23();
}

fn q1(){
    //  Q1: Write a Rust program, which accepts the radius of a circle from the user and computes the area.
    let mut radius = String::new();
    println!("******************* Calculate Area of a Circle *******************");
    print!("Enter Radius of Circle: ");
    io::stdout().flush().unwrap(); // to force the buffer to output before read_line is called
    io::stdin().read_line(&mut radius).unwrap();
    let num: f32 = radius.trim().parse().expect("Not a number");
    println!("Area of Circle having radius = {} is: {}", radius, (PI * num * num));
}

fn q2(){
    //  Q2: Write a Rust program to check if a number is positive, negative or zero
    println!("********* Check Number either positive, negative or zero *********");
    print!("Enter Number: ");
    let mut number = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number).unwrap();
    let num: i32 = number.trim().parse().expect("Not a number");
    if num == 0{
        println!("Number is Zero");
    }
    else if num > 0{
        println!("Number is Positive");
    }
    else if num < 0{
        println!("Number is Negative");
    }
    else{
        println!("Not a Number");
    }
}

fn q3(){
    // Q3: Write a Rust program to check whether a number is completely divisible by another number. Accept two integer
    // values form the user
    println!("***************** Divisibility Check of two numbers *****************");
    print!("Enter Numerator: ");
    let mut num1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut num1).unwrap();
    let numerator: f32 = num1.trim().parse().expect("Not a number");
    print!("Enter Denominator: ");
    let mut num2 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut num2).unwrap();
    let denominator: f32 = num2.trim().parse().expect("Not a number");
    if numerator % denominator == 0.0{
        println!("Number {} is completely divisible by {}", num1, num2);
    }
    else{
        println!("Number {} is not completely divisible by {}", num1, num2);
    }
}

fn q4(){
    // Q4: Write a Rust program to get the volume of a sphere, please take the radius as input from user
    println!("******************* Calculate Volume of a sphere *******************");
    print!("Enter Radius of Sphere: ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let radius: f32 = input.trim().parse().expect("Not a number");
    println!("Volume of Sphere having radius {} is {}", radius, 4.0 * PI * radius * radius);
}

fn q5(){
    // Q5: Write a Rust program to get a string which is n (non-negative integer) copies of a given string.
    println!("*********************** Copy string n times  ***********************");
    print!("Enter String: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let string: &str = &input1.trim();
    print!("How many copies of String you need: ");
    let mut input2 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input2).unwrap();
    let times: u16 = input2.trim().parse().expect("Not a Number");
    let mut text = String::from("");
    let mut x = 0;
    while x < times{
        x = x + 1;
        text.push_str(string);
    }
    println!("{} copies of {} are {}", times, string, text);
}

fn q6(){
    // Q6: Write a Rust program to find whether a given number (accept from the user) is even or odd, print out an appropriate message to the user.
    println!("****************** Check if number is Even or Odd ******************");
    print!("Ënter Number: ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let number: f32 = input.trim().parse().expect("Not a Number");
    if number % 2.0 == 0.0{
        println!("{} is Even", input);
    }
    else{
        println!("{} is Odd", input);
    }
}

fn q7(){
    // Q7: Write a Rust program to test whether a passed letter is a vowel or not
    println!("*************************** Vowel Tester ***************************");
    let vowels: [&str; 10] = ["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"];
    print!("Enter a Character: ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let character: &str = input.trim();
    let mut finded = String::from("Consonant");
    for i in vowels.iter(){
        if &character == i {
            finded = String::from("Vowel");
        }
    }
    println!("Letter {} is {}", character, finded);
}

fn q8(){
    // Q8: Write a Rust program that will accept the base and height of a triangle and compute the area
    println!("*************************** Triangle area ***************************");
    print!("Enter magnitude of Triangle Base: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let base: u32 = input1.trim().parse().expect("Not a Number");
    print!("Enter magnitude of Triangle Height: ");
    let mut input2 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input2).unwrap();
    let height: u32 = input2.trim().parse().expect("Not a Number");
    println!("Area of Triangle with Height {} and Base {} is {}", height, base, (height * base) / 2);
}

fn q9(){
    // Q9: Write a Rust program to compute the future value of a specified principal amount, rate of interest, and a number of years
    println!("************************ Calculate Interest ************************");
    print!("Enter Principal amount: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let principal_amount: f32 = input1.trim().parse().expect("Not a Number");
    print!("Enter Rate of interest in %: ");
    let mut input2 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input2).unwrap();
    let interest_rate: f32 = input2.trim().parse().expect("Not a Number");
    print!("Enter number of years for investment: ");
    let mut input3 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input3).unwrap();
    let years: f32 = input3.trim().parse().expect("Not a Number");
}

// fn q10(){
//     // Q10: Write a Rust program to compute the distance between the points (x1, y1) and (x2, y2).
//     println!("*********************** Euclidean distance ***********************");
//     print!("Enter Co-ordinate for x1: ");
//     let mut input1 = String::new();
//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut input1).unwrap();
//     let x1: u16 = input1.trim().parse().expect("Not a Number");
//     print!("Enter Co-ordinate for x2: ");
//     let mut input2 = String::new();
//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut input2).unwrap();
//     let x2: u16 = input2.trim().parse().expect("Not a Number");
//     print!("Enter Co-ordinate for y1: ");
//     let mut input3 = String::new();
//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut input3).unwrap();
//     let y1: u16 = input3.trim().parse().expect("Not a Number");
//     print!("Enter Co-ordinate for y2: ");
//     let mut input4 = String::new();
//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut input4).unwrap();
//     let y2: u16 = input4.trim().parse().expect("Not a Number");
//     let distance1 = x2 - x1;
//     let distance2 = y2 - y1;
// }

fn q11(){
    // Q11: Write a Rust program to convert height in feet to centimetres
    println!("****************** Feet to Centimeter Converter ******************");
    print!("Enter Height in Feet: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let height: f32 = input1.trim().parse().expect("Not a Number");
    println!("There are {} cm in {} feet", height * 30.48, height);
}

fn q12(){
    // Q12: Write a Rust program to calculate body mass index
    println!("***************** BMI(Body Mass Index) Calculator *****************");
    print!("Enter Height in cm: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let mut height: f32 = input1.trim().parse().expect("Not a Number");
    height = height / 100.0; // height in meters
    print!("Enter Weight in Kg: ");
    let mut input2 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input2).unwrap();
    let weight: f32 = input2.trim().parse().expect("Not a Number");
    println!("Your BMI is {}", weight / (height * height));
}

fn q13(){
    // Q13: Write a Rust program to sum of the first n positive integers
    println!("******************* Sum of n Positive Integers *******************");
    print!("Enter value of n: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let n: u16 = input1.trim().parse().expect("Not a Number");
    let mut sum: u16 = 0;
    let mut x: u16 = 0;
    while x <= n{
        sum = sum + x;
        x = x + 1;
    }
    println!("Sum of n Positive integers till {} is {}", n , sum);
}

fn q14(){
    // Q14: Write a Rust program to calculate the sum of the digits in an integer
    println!("********************* Digits Sum of a Number *********************");
    print!("Enter a number: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let num: u32 = input1.trim().parse().expect("Not a number");
    let arr = number_to_vec(&num);
    let mut sum: u32 = 0;
    for i in arr.iter(){
        sum = sum + i;
    }
    println!("Sum of digits {:?} is {}", arr, sum);
    fn number_to_vec(n: &u32) -> Vec<u32> {
        n.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect()
    }
}

fn q15(){
    // Q15: Write a Rust program to convert an decimal integer to binary
    println!("****************** Decimal to Binary Converter ******************");
    print!("Enter a decimal number: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let num: u32 = input1.trim().parse().expect("Not a number");
    println!("Binary Representation of {} is {:b}", num, num);
}

fn q16(){
    // Q16: Write a program to convert binary number to Decimal number
    println!("****************** Binary to Decimal Converter ******************");
    print!("Enter a Binary Number: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let string: &str = input1.trim();
    let num = isize::from_str_radix(&string, 2).unwrap();
    println!("Decimal Representation of {} is {}", string, num);
}

fn q17(){
    // Q17: Input a text and count the occurrences of vowels and consonant
    println!("***************** Vowel and Consonants Counter *****************");
    let vowels: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    print!("Enter text: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let text: &str = input1.trim();
    let mut no_of_vowels: u16 = 0;
    let mut no_of_consotants: u16 = 0;
    for x in text.chars(){
        if vowels.contains(&x){
            no_of_vowels = no_of_vowels + 1;
        }
        else{
            no_of_consotants = no_of_consotants + 1;
        }
    }
    println!("Number of Vowels: {}", no_of_vowels);
    println!("Number of Consotansts: {}", no_of_consotants);
}

fn q18(){
    // Q18: Write a program to check whether given input is palindrome or not
    println!("********************** Palindrome tester **********************");
    print!("Enter text: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let text: &str = input1.trim();
    if is_palindrome(&text){
        println!("Text {} is Palindrome", text);
    }
    else{
        println!("Text {} is Not a Palindrome", text);
    }
    fn is_palindrome(string: &str) -> bool {
        let half_len = string.len()/2;
        string.chars().take(half_len).eq(string.chars().rev().take(half_len))
    }
}

fn q19(){
    // Q19: Write a Rust program that accepts a string and calculate the number of digits and letters
    println!("******* Count Alphabets, Numbers and Special Characters *******");
    print!("Enter Text: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let text: &str = input1.trim();
    let mut no_of_digits: u16 = 0;
    let mut no_of_alphabets: u16 = 0;
    let mut no_of_special_chars: u16 = 0;
    let mut no_of_spaces: u16 = 0;
    for i in text.bytes(){
        if i >= 48 && i <= 57{
            no_of_digits = no_of_digits + 1;
        }
        else if (i >= 65 && i <= 90) || (i >= 97 && i <= 122){
            no_of_alphabets = no_of_alphabets + 1;
        }
        else if i == 32{
            no_of_spaces = no_of_spaces + 1;
        }
        else{
            no_of_special_chars = no_of_special_chars + 1;
        }
    }
    println!("Numbers = {}", no_of_digits);
    println!("Alphabets = {}", no_of_alphabets);
    println!("Spaces = {}", no_of_spaces);
    println!("Special Characters = {}", no_of_special_chars);
}

fn q20(){
    // Q20: Write a Rust program to construct the following pattern:
    // *
    // * *
    // * * *
    // * * * *
    // * * * * *
    // * * * *
    // * * *
    // * *
    // *
    print!("Enter number of Column: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let col: u16 = input1.trim().parse().expect("Not a Number");
    let mut x: u16 = 0;
    while x <= col{
        x = x + 1;
        for _i in 1..x{
            print!("*");
        }
        println!("");
    }
    while x > 0{
        x = x - 1;
        for _i in 1..x{
            print!("*");
        }
        println!("");
    }
}

fn q21(){
    // Q21: Write a Rust program to construct the following pattern:
    // 1
    // 1 2
    // 1 2 3
    // 1 2 3 4
    // 1 2 3 4 5
    // 1 2 3 4
    // 1 2 3
    // 1 2
    // 1
    print!("Enter number of Column: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let col: u16 = input1.trim().parse().expect("Not a Number");
    let mut x: u16 = 0;
    while x <= col{
        x = x + 1;
        for i in 1..x{
            print!("{}", i);
        }
        println!("");
    }
    while x > 0{
        x = x - 1;
        for i in 1..x{
            print!("{}", i);
        }
        println!("");
    }
}

fn q22(){
    // Q22: Write a Rust program to construct the following pattern:
    // 1
    // 22
    // 333
    // 4444
    // 55555
    // 666666
    // 7777777
    // 88888888
    // 999999999
    print!("Enter number of Rows: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let rows: u16 = input1.trim().parse().expect("Not a Number");
    let mut x: u16 = 1;
    while x <= rows{
        for _i in 0..x{
            print!("{}", x);
        }
        println!("");
        x = x + 1;
    }
}

fn q23(){
    // Q23: Write a Rust program to calculate number of days between two dates
    println!("*********************** Days Calculator ***********************");
    print!("Enter initial date in (dd/mm/yy) format: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    input1 = format!("{} 00/00/00", input1);
    let date1: &str = input1.trim();
    print!("Enter last date in (dd/mm/yy) format: ");
    let mut input2 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input2).unwrap();
    input2 = format!("{} 00/00/00", input2);
    let date2: &str = input2.trim();
    let d1 = Utc.datetime_from_str(&date1, "%d/%m/%Y %H/%M/%S").unwrap();
    let d2 = Utc.datetime_from_str(&date2, "%d/%m/%Y %H/%M/%S").unwrap();
    let duration = d2.signed_duration_since(d1);
    println!("There are {:?} days in between {} and {}", duration.num_days(), date1, date2);
}