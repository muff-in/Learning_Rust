// Bitwise Operators

fn main() {
    let a = 5;
    let b = 6;
    println!("Operand 1: {}, Operand 2: {}", a, b);
    println!("AND: {}", a & b);
    println!("OR: {}", a | b);
    println!("XOR: {}", a ^ b);
    println!("NOT a: {}", !a);
    println!("Left shift: {}", a << 2);
    println!("Right shift: {}", a >> 1);
}

// Output: Operand 1: 5, Operand 2: 6
// AND: 4
// OR: 7
// XOR: 3
// NOT a: -6
// Left shift: 20
// Right shift: 2