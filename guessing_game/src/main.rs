
use std::cmp::Ordering;
use std::io; 

use rand::Rng;

// this is known as the prelude  

fn main() {
    println!("Guess the number!");
    println!("Enter your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // The Rng trait defines methods that random number generators implement
    // rand::thread_rng() -  function for  local random generator to the current thread
    // gen_range method - range expression (start..=end) 
    println!("The Secret number  is : {secret_number}");
   
    loop {
    println!("Please enter your guess.");
    let mut guess = String::new();
    // let is  how variables are defined 
    // mut = mutable variable must be declared before the variable 
    // by default all variables are immutable 
    //
    // ' = '  means binding the variable 
    // String::new - a function that returns a new instance of String
    // String is a string type provided by the standard library that is growable, UTF-8 encoded bit
    // of text 
    //
    // a mutable variable that is currently bound to a new, empty instance of string is created.
    //

    io::stdin()
    // if prelude wasnt used  this could have been written as std:io:stdin
    //
        .read_line(&mut guess)
        // (&mut guess) - arguments of read_line 
        // & - reference  
        // like variables references are also immutable unless stated otherwise
        //
        .expect("Failed to read line");
        // RESULT is an enum which is a type that can be one of possible states.
        //We call each state a VARIANT
        // RESULT variants [ok, err]
        // 

    // could have been written all in one line 
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_)  => continue,
    };
    // Ok(num) -> if parse can successfully turn the string into a number then okay
    // (_) -> is a catch all value 
    // 
    //let guess: u32 = guess.trim().parse().expect("Please type a number:");
    // when taking input trim() is requred for read_line input - trims the \n for newline  
    // parse method converts string to another type 
    // the (:) will annotate the variable type
        // u32 - unsigned 32 bit number
        // u64 - unsigned 64 bit number
        // i32 - signed 32 bit number
        // i64 - signed 64 bit number

    println!("You guessed: {guess}");
    // printing values of a variable ("{variable}")   
    // printing expression ("{}", expression)
    //
    match guess.cmp(&secret_number)    {   
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You guessed it!");
            break;
            }

        }
    }
}   

