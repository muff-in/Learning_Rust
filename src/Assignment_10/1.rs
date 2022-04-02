// Character

// The variable is used to store a single character value, such as a single digit or a single alphabet.
// The value assigned to a char variable is enclosed in a single quote('') .

// To explicitly define a character variable, char keyword is used.
fn main() {
    let my_char: char = 'a';
    println!("char: {}", my_char);
}

// output: char: a

/*  Unlike some other languages, a character in Rust takes up 4 bytes rather than a single byte.
It does so because it can store a lot more than just an ASCII value like emojis, Korean, Chinese, and Japanese characters.*/
