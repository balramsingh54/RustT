use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

    // by default variable are immutable it means
    // mutable means : it's value can be changed 
    // immutable mean: it's value can't be changed

    let mut x= 5;
    println!("the value of x is {}", x);
    
    x= 6;
    println!("the value of x is {}", x);
}