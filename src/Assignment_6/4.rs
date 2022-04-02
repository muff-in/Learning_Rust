// Example of both local and global variables.

fn main() {
    let outer_variable = 112;
    let inner_variable = 213;
    {
        // start of code block
        println!("block variable inner: {}", inner_variable);
        println!("block variable outer: {}", outer_variable);
    } // end of code block
    println!("inner variable: {}", inner_variable);
}

// The variables declared using the const keyword can be declared in local as well as global scope.
