// An array is a homogenous sequence of elements.
// Being a compound type, it is used when the collection of values of the same type are to be stored in a single variable.

// In Rust, an array can only be of a fixed length.
// Like all other languages, each element in the array is assigned an index. By default, the first element is always at index 0.

// Define an Array

// let name_of_the_array: [type; size] = [ele1, ele2, ele3, ele4, ele5];

// type: the type of the elements in the array
// size: the number of elements in the array

fn main() {
    let array_of_ints: [i32; 5] = [1, 2, 3, 4, 5];
    let array_of_strings: [String; 5] = ["hello", "world", "!"];
    let array_of_characters: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
}
