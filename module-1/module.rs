fn add(num1: f32, num2: f32) -> f32 {
    num1 + num2
}

// #[derive(Debug)] is letting the compiler know that we would like it to automatically generate the Debug trait for us.
#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

/**
 * The default integer type in Rust is i32
 *
 * Every statement in Rust must end with a semicolon; expressions do not end with semicolons
 */
fn main() {
    const NUM1: i16 = 1;
    let num1 = f32::from(NUM1);
    println!("{}", add(num1, 2.0));

    // Anonymous function
    // Anonymous functions don't need type annotations like regular functions do. They are inferred! 🎉
    // we can write a anonymous function w/o type annotations as well:
    let sum = |a: i32, b: i32| a + b;
    println!("{}", sum(3, 20));

    let get_course_two = || {
        let blue = Colors::Blue;
        blue
    };

    const TOTAL_SALES: i32 = 100;
    let test_result = if TOTAL_SALES >= 100 {
        "nice job"
    } else {
        "you are a bad sales person"
    };
    println!("{}", test_result);

    println!("{:?}", get_course_two());

    let mut counter = 0;
    while counter <= 20 {
        println!("{}", counter);
        if counter == 10 {
            break;
        }
        counter += 1;
    }

    let get_course_one = || Colors::Blue;
    println!("{:?}", get_course_one());

    // Enum
    let color = Colors::Red;
    println!("{:?}", color);

    // call the match function
    expression_loops();

    // call the string_mod function
    string_mod();

    // call structs and methods function
    struct_methods();
}

enum Status {
    Online,
    Offline,
    Away,
    Failure(String),
}

fn expression_loops() {
    let lan = Status::Failure("Error".to_string());
    let msg = "Error";
    match lan {
        Status::Online => println!("Online"),
        Status::Offline => println!("Offline"),
        Status::Away => println!("Away"),
        Status::Failure(msg) => println!("{}", msg),
    }
}

fn string_mod() {
    let s = "Hello Amit, how are you?";
    println!("{}", s);

    let s_test = &s[0..5];
    println!("{}", s_test);

    let greetings: &str = "I am fine, thank you!";
    println!("{}", greetings);

    let poem = "
    nice one
    nice one
    nice one
    ";

    println!("{}", poem);

    let raw_str = r"tell me something \ that you about the character";
    println!("{}", raw_str);

    let str1 = "Hello";
    let str2: &str = "Hello";

    if str1 == str2 {
        println!("str1 and str2 are equal");
    } else {
        println!("str1 and str2 are not equal");
    }

    let mut name = String::new();
    name.push_str("Amit");
    name.push_str("Shoni");
    println!("{}", name);

    let string_one = String::from("Amit");
    let string_two = String::from("Devika");

    let concatenated_string = string_one + " " + &string_two;
    println!("{}", concatenated_string);

    let result: Vec<&str> = concatenated_string.split_whitespace().collect();
    println!("{:?}", result);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Structs are a rough equivalent of creating classes in JavaScript. You create a Struct when you need a complex data type of your own.
#[derive(Debug)]
struct URL {
    protocol: String,
    hostname: String,
    pathname: String,
}

impl URL {
    fn toString(&self) -> String {
        format!("{}://{}/{}", self.protocol, self.hostname, self.pathname)
    }
    fn from(url: &str) -> URL {
        let string = String::from(url);
        let vec: Vec<&str> = string.split("://").collect();
        let protocol = String::from(vec[0]);
        let rest = String::from(vec[1]);
        let vec2: Vec<&str> = rest.split("/").collect();
        let hostname = String::from(vec2[0]);
        let pathname = String::from(vec2[1]);
        URL {
            protocol,
            hostname,
            pathname,
        }
    }
}

fn struct_methods() {
    let name = String::from("Amit");
    let age = 31;

    let me = Person { name, age };

    let modified_me = Person { age: 20, ..me };
    println!("{:?}", modified_me);

    let app = URL::from("https://app.rust-for-js.dev/posts/07-structs-and-methods/");
    println!("{:?}", app);
}
