// if…else if…else Expression

// In an if…else if…else construct, if the condition within the if expression evaluates to be false, then the statement within the else if block is executed.

/* if condition {
    statement1;
    statement2;
}
else if condition {
    statement1;
    statement2;
}
else {
    statement1;
    statement2;
} */

fn main() {
    //define a variable
    let learn_language = "Rust";
    // if..elseif..else construct
    if learn_language == "Java" {
        println!("You are learning Rust language!");
    } else if learn_language == "Rust" {
        println!("You are learning Rust language!");
    } else {
        println!("You are learning some other language!");
    }
}

// output: You are learning Rust language!
