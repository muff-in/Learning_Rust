fn main() {
    let mut n = 0; // Initialize n

    loop {
        // Repeat forever
        n += 1; // Increment n

        if n == 7 {
            // If n is 7
            continue; // Skip the rest of the loop
        }
        println!("{}", n); // Print n

        if n == 10 {
            // If n is 10
            break; // Break out of the loop
        }
    } // End of loop
}
