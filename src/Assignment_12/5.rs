
// Mutable Tuples

// Just like a variable becomes mutable by adding the mut keyword after let, the same goes for a tuple.

fn main() {
    
    let mut data = ("C", 101, "86k", 'J'); //define a tuple
    
    println!("The value of the tuple at index 0 and index 1 are {} {}", data.0, data.1); //print the value of tuple
   
    data.0 = "Rust";  //modify the value at index 0
  
    println!("The value of the tuple at index 0 and index 1 are {} {}", data.0, data.1);   //print the modified value
}

// output: The value of the tuple at index 0 and index 1 are C 101
// The value of the tuple at index 0 and index 1 are Rust 101