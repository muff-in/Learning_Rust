// Shorthand if

// Instead of writing a lengthy if-else construct, we can use a shorthand if.

// Syntax: 

// let k=if conditon {statement} else {statement}; 

// In above code we are firstly declaring a variable k and then we are using if condition to check the condition and if condition is true then we are executing the statement otherwise we are executing the statement.

// Note: This is similar to a ternary operator in languages like C and C++.

fn main() {
    
    let learn_language = "Rust";
     
    let res= if learn_language == "Rust" {"You are learning Rust language!"} else {"You are learning some other language!"};
     // Here we are using if condition to check the condition and if condition is true then we are executing the statement otherwise we are executing the statement.
    
     println!("{}", res);
}

// Output: You are learning Rust language!