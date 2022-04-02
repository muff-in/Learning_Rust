// Conditional Expression

// Conditions basically give the power to make decisions. They check the condition of the expression and act accordingly.

// There are three types of conditional expression in Rust:

// 1. if Expression

// 2. if let Expression

// 3. match Expression

// if Expression

fn main() {
    //define a variable
    let learn_language = "Rust";
    // if construct
    if learn_language == "Rust" {
        println!("You are learning Rust language!");
    }
}

// The if expression is used to check the condition and if the condition is true then it will execute the code inside the block.

// output: You are learning Rust language!
