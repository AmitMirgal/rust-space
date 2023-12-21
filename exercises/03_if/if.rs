use std::env;

fn main() {
    // get command-line arguments as an iterator
    let args: Vec<String> = env::args().collect();

    // print the entire command-line arguments.
    // The {:?} format specifier is used to print a value using the Debug trait.
    println!("All command-line arguments: {:?}", args);

    let number = if args.len() > 1 {
        // parse string to int
        // Ok is an enum variant that is part of the Result type.
        let _number = if let Ok(parsed_number) = args[1].parse::<i8>() {
            println!("parsed_number value is {}", parsed_number);
            parsed_number
        } else {
            3
        };
        _number
    } else {
        3
    };

    // simple if-else condition
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // returning value from if-else condition
    let result = if number < 5 { number + 2 } else { number + 5 };

    println!("result is {} ", result)
}
