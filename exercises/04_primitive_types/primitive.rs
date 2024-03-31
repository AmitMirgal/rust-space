// notes in primitive.md
fn main() {
    // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters
    // Rust has two primitive compound types: tuples and arrays.
    //
    // it has a collection of similar or different data type values
    let tup = (500, "6.4a", "1");
    let (a1, a2, _) = tup;
    println!("tuple values are => {}, {}", a1, a2);

    // array is a collection of same type of data
    let a = [1, 2, 3, 4, 5];
    println!("array value is => {:?}", a);
    println!("array value at index 0 is => {:?}", a[0]);
}
