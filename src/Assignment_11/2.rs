
// Access elements in an array

// Any value of the array can be accessed by writing the array name followed by the index number enclosed within square brackets [ ].

fn main(){
    let array_of_ints: [i32; 5] = [1, 2, 3, 4, 5]; // define an array of 5 integers

    println!("First value: {}", array_of_ints[0]); // print the first value.. index starts at 0
    println!("Second value: {}", array_of_ints[1]); // print the second value
    println!("Third value: {}", array_of_ints[2]); // print the third value
    println!("Fourth value: {}", array_of_ints[3]); // print the fourth value
    println!("Fifth value: {}", array_of_ints[4]); // print the fifth value
}

/* output: First value: 1
Second value: 2
Third value: 3
Fourth value: 4
Fifth value: 5 */