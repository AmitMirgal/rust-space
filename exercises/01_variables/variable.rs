fn main() {
    let x = 10;
    /*
    const requires explicit type annotation for constants.
    let allows for type inference when declaring variables.
    */
    const NUMBER: u8 = 200;
    println!("variables value are {}, {}", x, NUMBER);

    // mut keyword is used to declare mutable variables
    let mut mutate_value = 2;
    println!("variables value are {}", mutate_value);

    mutate_value = 123;
    println!("variables value are {}", mutate_value);

    let change_value = 231;
    println!("variables value are {}", change_value);
}
