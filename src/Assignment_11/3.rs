
// Mutable Arrays

// Just like a variable becomes mutable by adding the mut keyword after let, the same goes for an array.

fn main(){

    let mut array_of_ints: [i32; 5] = [1, 2, 3, 4, 5]; // define an array of 5 integers that is mutable
    println!("Fourth value: {}", array_of_ints[3]); // print the first value.. index starts at 0

    array_of_ints[3] = 10; // change the value of the fourth element
    println!("Fourth value: {}", array_of_ints[3]); // print the fourth value
}

/* output: Fourth value: 4
    Fourth value: 10 */