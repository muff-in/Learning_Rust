
// Logical operators operate on true / false values.

// The && operator is used to evaluate two conditions and returns true if both conditions are true.
// The || operator is used to evaluate two conditions and returns true if either of the conditions is true.
// 	The operator returns the inverse of the expression’s result.

fn main() {
    let a = true;
    let b = false;
    println!("Operand 1:{}, Operand 2:{}", a, b); // print the operands
    println!("AND:{}", a && b); // print the AND of the operands
    println!("OR:{}", a || b); // print the OR of the operands
    println!("NOT:{}", !a); // print the NOT of the operands
}

// output: Operand 1:true, Operand 2:false
// AND:false
// OR:true
// NOT:false