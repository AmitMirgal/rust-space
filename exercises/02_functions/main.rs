// mod -> module
mod util;
// unlike in JS, where we use the 'import' keyword to import a function.
use util::math::add;

fn main() {
    call_me(3);
    let result = get_number();
    println!("result is {} ", result);

    let add_result = add(1, 2);
    println!("additiona result is {} ", add_result);
}

fn call_me(num: i8) {
    for i in 0..num {
        println!("num is {} ", i + 1);
    }
}

fn get_number() -> i8 {
    2
}
