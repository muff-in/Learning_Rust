// Global Variable
// Global variables are declared outside of any function and have a scope limited to the entire program.

fn main() {
    let x = 5; // declare a global variable, x, and initialize it with a value of 5.
    {
        println!("Number: {}", x); // print the value of x
    }
}

// output: Number: 5
