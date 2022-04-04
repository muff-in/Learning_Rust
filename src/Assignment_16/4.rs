// Nested if Expression

// An if expression inside the body of another if expression is referred to as a nested if expression.

// An if construct is enclosed within an if construct. The general syntax is:

/*
if codition {

    if condition {
        statement
    }
}
*/

fn main() {
    let learn_language1 = "Rust";
    let learn_language2 = "Java";

    if learn_language1 == "Rust" {
        //outer if
        if learn_language2 == "Java" {
            // inner if
            println!("You are learning Rust and Java language!");
        }
    } else {
        println!("You are learning some other language!");
    }
}

// In the above example, the outer if statement is evaluated first. If the condition is true, the inner if statement is evaluated.
//  If the condition is false, the else statement is executed.

// Output: You are learning Rust and Java language!


//  The nested if expression can also be written with a AND expression in an if.

/* if condition1 && condition2 
{
 //statement
}

*/

// This is true only if the second if statement is the only thing inside the first if.