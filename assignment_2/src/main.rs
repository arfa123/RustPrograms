use std::iter;
use std::io;
use std::io::Write;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    println!("******************* Rust Programming Assignment Set 2 *******************");
    q1();
    q2();
    q3();
    q4();
    q5();
    q6();
    q7();
    q8();
    q9();
    q10();
    q11();
    q12();
}

fn q1(){
    // Q1: Generate random floating point and integer numbers with the help of random-number generator rand::Rng obtained via rand::thread_rng.
    println!("********************** Q1: Generate Random Numbers **********************");
    let mut rng = thread_rng();
    let x8: u8 = rng.gen();
    let x16: u16 = rng.gen();
    let x32: u32 = rng.gen();
    let x64: u64 = rng.gen();
    let x128: u128 = rng.gen();
    let s8: i8 = rng.gen();
    let s16: i16 = rng.gen();
    let s32: i32 = rng.gen();
    let s64: i64 = rng.gen();
    let s128: i128 = rng.gen();
    let float32: f32 = rng.gen();
    let float64: f64 = rng.gen();
    let boolean: bool = rng.gen();
    println!("Random u8: {}", x8);
    println!("Random u16: {}", x16);
    println!("Random u32: {}", x32);
    println!("Random u64: {}", x64);
    println!("Random u128: {}", x128);
    println!("Random i8: {}", s8);
    println!("Random i16: {}", s16);
    println!("Random i32: {}", s32);
    println!("Random i64: {}", s64);
    println!("Random i128: {}", s128);
    println!("Random f32: {}", float32);
    println!("Random f64: {}", float64);
    println!("Random bool: {}", boolean);
}

fn q2(){
    // Q2: Generate a random integer and float value within half-open (0, 10) range (not including 10) with Rng::gen_range
    println!("************** Q2: Generate Random Numbers within a Range **************");
    let mut rng = thread_rng();
    let integer: u8 = rng.gen_range(0, 10);
    println!("Integer: {}", integer);
    let float: f32 = rng.gen_range(0.0, 10.0);
    println!("Float: {}", float);
}

fn q3(){
    // Q3: Randomly generate a string of 30 ASCII characters in the range A-Z, a-z, 0-9, with Alphanumeric sample.
    println!("********************* Q3: Generate a Random String *********************");
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(30)
            .collect();
    println!("Random string: {}", chars);
}

fn q4(){
    // Q4: Sort a given Vector of integers: [1, 10, 5, 2, 15]
    println!("********************* Q4: Sort a Vector of Integers *********************");
    let mut v = [1, 10, 5, 2, 15];
    v.sort();
    println!("{:?}", v);
}

fn q5(){
    // Q5: Sort a given Vector of floats: [1.1, 1.15, 5.5, 1.123, 2.0]
    println!("********************** Q5: Sort a Vector of Floats **********************");
    let mut floats = [1.1, 1.15, 5.5, 1.123, 2.0];
    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", floats);
}

fn q6(){
    // Q6: Generate a Vector of 100 random floats between 0 and 900. Find out how many of the random numbers are between 0 to 300, 300 to 600, and above 600.
    println!("************* Q6: Generate a Float Vector and Count Elements *************");
    let mut float_vec: Vec<f32> = Vec::new();
    let mut rng = thread_rng();
    let mut x: u8 = 0;
    while x < 100{
        x = x + 1;
        float_vec.push(rng.gen_range(0.0, 900.0));
    }
    let mut below_300: u8 = 0;
    let mut bet_300_600: u8 = 0;
    let mut above_600: u8 = 0;
    for i in float_vec.iter(){
        if i <= &300.0{
            below_300 = below_300 + 1;
        }
        else if (i > &300.0) & (i <= &600.0){
            bet_300_600 = bet_300_600 + 1;
        }
        else if i > &600.0{
            above_600 = above_600 + 1;
        }
    }
    println!("Number of floats between 0 and 300 is {}", below_300);
    println!("Number of floats between 3000 and 600 is {}", bet_300_600);
    println!("Number of floats above 600 is {}", above_600);
}

fn q7(){
    // Q7: Generate a random integer vector between 0 to 100 and keep on adding numbers to the vector until a number greater than 90 is generated. Once the vector is generated sort it.
    println!("**** Q7: Generate a Integer Vector until a certain condition is met ****");
    let mut int_vec: Vec<u8> = Vec::new();
    let mut rng = thread_rng();
    loop{
        let i: u8 = rng.gen_range(0, 100);
        if i > 90{
            break int_vec.push(i)
        }
        else{
            int_vec.push(i);
        }
    }
    int_vec.sort();
    println!("{:?}", int_vec);
}

fn q8(){
    // Q8: Build a number-guessing game to generate a random integer number from 1 to 10 and have the user try to guess that. Tell them if they get it right or wrong, and if they get it wrong, show them what the random number was.
    println!("*********************** Q8: Number Guessing Game ***********************");
    let mut rng = thread_rng();
    let random_number: u8 = rng.gen_range(1, 10);
    print!("I'm thinking of a number from 1 to 10.\nYour guess: ");
    let mut input1 = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();
    let num: u8 = input1.trim().parse().expect("Not a number");
    if num == random_number{
        println!("That's right! My secret number was {}!", random_number);
    }
    else{
        println!("Sorry, but I was really thinking of {}.", random_number);
    }
}

fn q9(){
    // Q9: Write a program that simulates a dice roll by picking a random number from 1-6 and then picking a second random number from 1-6. Add the two values together, and display the total.
    println!("******************************* Q9: Dice ********************************");
    let mut rng = thread_rng();
    let random_number1: u8 = rng.gen_range(1, 6);
    let random_number2: u8 = rng.gen_range(1, 6);
    println!("HERE COMES THE DICE!\nRoll#1: {}\nRoll#2: {}\nThe Total is {}!", random_number1, random_number2, random_number1 + random_number2);
}

fn q10(){
    // Q10: Write a program that picks a random number from 1-100. The user keeps guessing as long as their guess is wrong,
    // and they've guessed less than 7 times. If their guess is higher than the number, say "Too high." If their guess is
    // lower than the number, say "Too low." When they get it right, the game stops. Or, if they hit seven guesses, the
    // game stops even if they never got it right.
    println!("********************* Q10: Hi-Lo with Limited Tries *********************");
    let mut rng = thread_rng();
    let random_number: u8 = rng.gen_range(1, 100);
    println!("I'm thinking of a number between 1-100. You have 7 guesses.");
    let mut x: u8 = 1;
    while x <= 7{
        print!("Guess #{}: ", x);
        let mut input1 = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input1).unwrap();
        let guessed: u8 = input1.trim().parse().expect("Not a number");
        if (x == 7) & (x != random_number){
            println!("Sorry, you didn't guess it in 7 tries. You lose. The number is {}", random_number);
        }
        else if guessed == random_number{
            println!("You guessed it! What are the odds?!?");
        }
        else if guessed > random_number{
            println!("Sorry, that guess is too high.");
        }
        else if guessed < random_number{
            println!("Sorry, you are too low.");
        }
        x = x + 1;
    }
}

fn q11(){
    // Q11: Write a program that gets several integers from the user. Sum up all the integers they give you. Stop looping when they enter a 0. Display the total at the end.
    println!("********************** Q11: Adding Values in a Loop **********************");
    let mut sum: isize = 0;
    loop{
        print!("Number: ");
        let mut input1 = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input1).unwrap();
        let num: isize = input1.trim().parse().expect("Not a number");
        sum = sum + num;
        println!("The total so far is {}", sum);
        if num == 0{
            break
        }
    }
}

fn q12(){
    // Q12: Write a calculator program. A minimal calculator will support the following functions:
    // • numbers with decimals (not just integers)
    // • addition (1 + 2 is 3)
    // • subtraction (12 - 4 is 8)
    // • multiplication (33 * 2 is 66)
    // • division (3 / 8 is 0.375)
    // • exponents (2 ^ 3 is 8)
    // • error messages when you do something wrong
    // Your calculator should keep on running until explicitly told to quit. I suggest typing a zero as the first operand to
    // cause it to quit,
    println!("************************ Q12: Project Calculator ************************");
    #[derive(Debug)]
    struct Exp{
        first_digit: f64,
        second_digit: f64,
        operation: String,
        error: String
    }
    loop{
        print!("Enter Expression: ");
        let mut input1 = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input1).unwrap();
        input1 = input1.trim().to_string();
        let result = evaluate(input1);
        if result.error.len() == 0{
            if result.first_digit == 0.0{
                break
            }
            if result.operation == "+"{
                println!("{} + {} is {}", result.first_digit, result.second_digit, result.first_digit + result.second_digit);
            }
            else if result.operation == "-"{
                println!("{} - {} is {}", result.first_digit, result.second_digit, result.first_digit - result.second_digit)
            }
            else if result.operation == "*"{
                println!("{} * {} is {}", result.first_digit, result.second_digit, result.first_digit * result.second_digit)
            }
            else if result.operation == "/"{
                println!("{} / {} is {}", result.first_digit, result.second_digit, result.first_digit / result.second_digit)
            }
        }
        else{
            println!("{}", result.error);
            break
        }
    }
    
    fn evaluate(exp: String) -> Exp {
        let numbers: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let operations: [char; 4] = ['+', '-', '*', '/'];
        let mut first = String::new();
        let mut second = String::new();
        let mut operation = String::new();
        let mut error = String::new();
        let mut first_done: bool = false;
        let mut operator_done: bool = false;
        let mut first_digit: f64 = 0.0;
        let mut second_digit: f64 = 0.0;
        for c in exp.chars(){
            if numbers.contains(&c){
                if first_done{
                    second.push(c);
                }
                else{
                    first.push(c);
                }
            }
            else if operations.contains(&c) & !operator_done{
                if first.len() == 0{
                    error = String::from("You entered invalid expression!");
                }
                else{
                    operation.push(c);
                    first_done = true;
                    operator_done = true;
                }
            }
        }
        if error.len() == 0{
            first_digit= first.trim().parse().expect("Not a Number");
            second_digit = second.trim().parse().expect("Not a Number");
        }
        let expression = Exp{
            first_digit,
            second_digit,
            operation,
            error
        };
        expression
    }
}