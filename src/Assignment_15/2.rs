// Compound Assignment Operator

// The compound assignment operator is used to perform an operation and then assign that value to the operand.

fn main() {
    let mut a = 2;
    println!("a:{}", a); // print the value of a
    a += 1; // add 1 to a
    println!("a+=1:{}", a); // print the value of a
    println!("a:{}", a); // print the value of a
    a -= 1; // subtract 1 from a
    println!("a-=1:{}", a); // print the value of a
    println!("a:{}", a); // print the value of a
    a /= 1; // divide a by 1
    println!("a/=1:{}", a); // print the value of a
    println!("a:{}", a); // print the value of a
    a *= 3; // multiply a by 3
    println!("a*=3:{}", a); // print the value of a
}

// output: a:2
// a+=1:3
// a:3
// a-=1:2
// a:2
// a/=1:2
// a:2
// a*=3:6