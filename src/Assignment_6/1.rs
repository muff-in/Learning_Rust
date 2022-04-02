// Scope of a variable

/* The scope of a variable refers to the visibility of a variable, or , which parts of a program can access that variable.
It all depends on where this variable is being declared.
If it is declared inside any curly braces {}, i.e., a block of code, its scope is restricted within the braces, otherwise the scope is global.*/

// Types of Variables

// Local Variable
// Local variables are declared within a function and have a scope limited to the function.
// After the closing curly brace, } , the variable is freed and memory for the variable is deallocated.

fn main() {
    let x = 5; // declare a local variable, x, and initialize it with a value of 5.
    println!("Number: {}", x); // print the value of x
}

// output: Number: 5

// Above code is an example of local variable.
// As you can see, the variable x is declared within the function main() and has a scope limited to the function.
