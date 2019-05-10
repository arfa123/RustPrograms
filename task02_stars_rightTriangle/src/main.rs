use std::io;

fn main() {
    println!("Enter Number of lines to print * right triangle:");
    let mut inp_val = String::new();
    io::stdin().read_line(&mut inp_val);
    let num = inp_val.trim().parse().unwrap();
    let mut x = 0;
    while x <= num{
        x = x + 1;
        for num in (1..x){
            print!("*");
        }
        println!("");
    }
}
