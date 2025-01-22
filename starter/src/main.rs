use std::io; // o obtain user input and then print the result as output, we need to bring the io input/output library into scope. The io library comes from the standard library, known as std:

//The "fn" syntax declares a new function; the parentheses, (), indicate there are no parameters; and the curly bracket, {, starts the body of the function.
fn main() {
    println!("Hello, cargo!"); // simple printing in rust (As you also learned in Chapter 1, println! is a macro that prints a string to the screen)
//We use the "let"statement to create the variable (let orange 10;)
// Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change
//let apples = 5; ** immutable
//let mut bananas = 5; ** mutable (vars with mut are mutable)

    let mut guess = String::new(); //String::new, a function that returns a new instance of a String. String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
    // This new function creates a new, empty string.
    
    io::stdin()
        .read_line(&mut guess)// read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value. Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states
        .expect("Failed to read line");
    //Result’s variants are Ok and Err

    println!("You guessed: {}", guess); // same as f-string in python (mixing string and other data types)
}

//Enums and Pattern Matching
// In this chapter, we’ll look at enumerations, also referred to as enums. Enums allow you to define a type by enumerating its possible variants.
//  First we’ll define and use an enum to show how an enum can encode meaning along with data. Next, we’ll explore a particularly useful enum, called Option,
//  which expresses that a value can be either something or nothing. Then we’ll look at how pattern matching in the match expression makes it easy to run different code for different values of an enum.
//  Finally, we’ll cover how the if let construct is another convenient and concise idiom available to handle enums in your code.