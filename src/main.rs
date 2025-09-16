//to get user input and generate output we need input output library comes from standard library std
use std::io;
use rand::Rng;
//Rng defines methods that random number generators implement, and this trait must be in scope for us to use those methods
use std::cmp::Ordering;
//The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.
//main function is the entry point of the program
fn main(){

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //rand::thread_rng function gives us the particular random generator
    //gen_range is defined by the Rng trait that we brought into scope with the use rand::Rng; statement. 
    //The gen_range method takes a range expression as an argument and generates a random number in the range.
    //The kind of range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds, so we need to specify 1..=100 to request a number between 1 and 100.


    
    println!("The secret number is: {secret_number}");

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

let guess: u32 = guess.trim().parse().expect("Please type a number!");
//Rust allows us to shadow the previous value of guess with a new one. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess, for example.
//The guess in the expression refers to the original guess variable that contained the input as a string.
//The trim method on a String instance will eliminate any whitespace at the beginning and end, which we must do before we can convert the string to a u32, which can only contain numerical data. The user must press enter to satisfy read_line and input their guess, which adds a newline character to the string. For example, if the user types 5 and presses enter, guess looks like this: 5\n. The \n represents “newline.” (On Windows, pressing enter results in a carriage return and a newline, \r\n.) The trim method eliminates \n or \r\n, resulting in just 5.
//The parse method on strings converts a string to another type.
println!("You guessed: {guess}");

//When printing the result of evaluating an expression, place empty curly brackets in the format string, then follow the format string with a comma-separated list of expressions to print in each empty curly bracket placeholder in the same order.

//eg:let x = 5;
//let y = 10;
//println!("x = {x} and y + 2 = {}", y + 2);
//This code would print x = 5 and y + 2 = 12.

 match guess.cmp(&secret_number){
  //The cmp method compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with: here it’s comparing guess to secret_number.
  //We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number.
     Ordering::Less => println!("Too small!"),
     Ordering::Greater => println!("Too big!"),
     Ordering::Equal => println!("You win!!"),
     //A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn. Patterns and the match construct are powerful Rust features: they let you express a variety of situations your code might encounter and they make sure you handle them all. 
 }


}