/* Constant variables are ones that are declared constant throughout the program scope, meaning, their value cannot be modified.
They can be defined in global and local scope. */

//They are declared using the const keyword followed by the name of the variable, colon (:), and then the data type of the variable.

// const name: i32 = 100;

// ---------------------------------------------------------------------------------------------------------------------

const ID_1: i32 = 4; // define a global constant variable
fn main() {
    const ID_2: u32 = 3; // define a local constant variable
    println!("ID:{}", ID_1); // print the global constant variable
    println!("ID:{}", ID_2); // print the local constant variable
}

// The value of a constant variable can be changed, but it cannot be reassigned.

// output: ID:4
// output: ID:3
