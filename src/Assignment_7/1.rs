// Variable shadowing is a technique in which a variable declared within a certain scope has the same name as a variable declared in an outer scope. This is also known as masking.
// This outer variable is said to be shadowed by the inner variable, while the inner variable is said to mask the outer variable.

fn main() {
    let outer_variable = 112;
    {
        // start of code block
        let inner_variable = 213;
        println!("block variable: {}", inner_variable);
        let outer_variable = 117; //
        println!("block variable outer: {}", outer_variable);
    } // end of code block
    println!("outer variable: {}", outer_variable);
}

// output: block variable: 213
// block variable outer: 117
// outer variable: 112

// Here, the variable inner_variable is declared within the code block and has the same name as the variable outer_variable.
