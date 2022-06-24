fn main() {
    let price = 129;
    let tax = 23.22;
    let total = f64::from(price) + tax;

    println!("{}", total);

    const SALES: i32 = 100;
    let price = get_price();
    let total = f64::from(SALES) + price;
    println!("{} {}", SALES, total);

    let mut array_test = [0; 5];
    println!("{:?}", array_test);
}

fn get_price() -> f64 {
    return 10.20;
}
