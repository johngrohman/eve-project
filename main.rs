use rand::Rng;
use std::io::Write;
mod eve;

fn main() {
    // Purpose:    Driver for DH problems
    // Parameters: None
    // User Input: If no args, input dec numbers
    // Prints:     If no args, then print result
    // Returns:    Nothing
    // Modifies:   Nothing
    // Calls:      ?
    // Tests:      arg_tests/ and stdio_tests/
    // Status:     Student does this

    //Initiate string variables
    let mut alice_broadcasts = String::new();
    let mut bob_broadcasts = String::new();
    let mut public_base = String::new();

    //If there are no arguments, then get input from user
    if std::env::args().len() != 3 {
        std::io::stdin()
            .read_line(&mut alice_broadcasts)
            .expect("Failed to read line");
        std::io::stdin()
            .read_line(&mut bob_broadcasts)
            .expect("Failed to read line");
        std::io::stdin()
            .read_line(&mut public_base)
            .expect("Failed to read line");

        // Convert the string variables into unsigned ints
        let alice_broadcasts: u64 = alice_broadcasts
            .trim()
            .parse()
            .expect("Please type a number!");
        let bob_broadcasts: u64 = bob_broadcasts
            .trim()
            .parse()
            .expect("Please type a number!");
        let public_base: u64 = public_base.trim().parse().expect("Please type a number!");

        let mut result = eve::baby_eve(alice_broadcasts, bob_broadcasts, public_base);
        println!("{} {} {}", result[0], result[1], result[2]);
    } else {
        // read in the input and output files from the arguments
        let args: Vec<String> = std::env::args().collect();
        let input_file = &args[1];
        let output_file = &args[2];

        // open the input file
        let input = std::fs::File::open(input_file).expect("Failed to open input file");
        let input_string = std::fs::read_to_string(input_file).expect("Failed to read input file");

        // parse the input_string
        let input_vec: Vec<&str> = input_string.split_whitespace().collect();
        let alice_broadcasts: u64 = input_vec[0].parse().expect("Please type a number!");
        let bob_broadcasts: u64 = input_vec[1].parse().expect("Please type a number!");
        let public_base: u64 = input_vec[2].parse().expect("Please type a number!");

        // open the output file
        let mut output = std::fs::File::create(output_file).expect("Failed to create output file");

        // call the big_eve function and write the result to the output file
        let result = eve::baby_eve(alice_broadcasts, bob_broadcasts, public_base);
        let result_string = format!("{} {} {}", result[0], result[1], result[2]);
        output
            .write_all(result_string.as_bytes())
            .expect("Failed to write to output file");
    }
}
