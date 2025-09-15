//to get user input and generate output we need input output library comes from standard library std
use std::io;

//main function is the entry point of the program
fn main(){

    println!("Guess the number!");

    println!("Please input your guess.");

    //in rust the variables are immutable by nature, i.e once a value is assigned to the variable it doesn't change again
    //to make a variable mutable we add mut to it
    //String::new represents the type of datatype being used,it represents a new instance of string is been introduced
    //:: in syntax represents that new is an associated function in the string type
    //new function creates a new empty string  
      let mut guess = String::new();

    //The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
    //read_line tells you what string to store
    //& indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
    //like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.
      io::stdin()
             .read_line(&mut guess)
             .expect("Failed to read line!");

//we could have also written it as: io::stdin().read_line(&mut guess).expect("Failed to read line");
//read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value. Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states.
//Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and it contains the successfully generated value. The Err variant means the operation failed, and it contains information about how or why the operation failed.
//When printing the value of a variable, the variable name can go inside the curly brackets.
println!("You guessed: {guess}");

//When printing the result of evaluating an expression, place empty curly brackets in the format string, then follow the format string with a comma-separated list of expressions to print in each empty curly bracket placeholder in the same order.

//eg:let x = 5;
//let y = 10;
//println!("x = {x} and y + 2 = {}", y + 2);
//This code would print x = 5 and y + 2 = 12.


}