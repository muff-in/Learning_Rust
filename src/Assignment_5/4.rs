// Assigning multiple variables


fn main () {
let (firstvar, secondvar) = ("Rust", "Language"); // we can assign multiple variables at once.

// let is the identifier and (firstvar, secondvar) is the variable name after thatt we use equals sign to assign values to the variables. respectively.

println!("{} {}", firstvar, secondvar); // print the values of the variables

}


// output: Rust Language

/* Tip:  If a variable is kept un-assigned or unused, you’ll get a warning. 
To remove such a warning write the expression #[allow(unused_variables, unused_mut)] at the start of the program code. 
However, it’s not a good practice to keep unassigned/unused variables. */
