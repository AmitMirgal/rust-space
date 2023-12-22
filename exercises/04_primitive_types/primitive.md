# 🦀 Primitive Types

## The primitive types in Rust and some common data types:

| Primitive Type | Example | Size (bytes) | Description |
| -------------- | ------- | ------------ | ----------- |
| `i8`           | `42i8`  | 1            | 8-bit signed integer |
| `i16`          | `42i16` | 2            | 16-bit signed integer |
| `i32`          | `42i32` | 4            | 32-bit signed integer |
| `i64`          | `42i64` | 8            | 64-bit signed integer |
| `u8`           | `42u8`  | 1            | 8-bit unsigned integer |
| `u16`          | `42u16` | 2            | 16-bit unsigned integer |
| `u32`          | `42u32` | 4            | 32-bit unsigned integer |
| `u64`          | `42u64` | 8            | 64-bit unsigned integer |
| `isize`        | `42`    | platform     | Signed integer (depends on the platform) |
| `usize`        | `42`    | platform     | Unsigned integer (depends on the platform) |
| `f32`          | `42.0`  | 4            | 32-bit floating-point number |
| `f64`          | `42.0`  | 8            | 64-bit floating-point number |
| `char`         | `'a'`   | 4            | Unicode character |
| `bool`         | `true`  | 1            | Boolean (either `true` or `false`) |
| `str`          | `"hello"`| -           | String slice (UTF-8 encoded) |


Rust is a statically-typed language, and its type system helps ensure memory safety without sacrificing performance. The usize and isize types depend on the platform's architecture and are used for indexing and representing sizes of data structures. The str type represents a string slice, which is a reference to a sequence of UTF-8 encoded bytes and is often used in conjunction with the String type for string manipulation.


****************************************************************************

## Compound types in Rust:

| Compound Type | Example Declaration | Example Initialization | Description |
| ------------- | -------------------- | ----------------------- | ----------- |
| Array         | `let arr: [i32; 3] = [1, 2, 3];` | - | Fixed-size array of elements of the same type |
| Tuple         | `let tuple: (i32, f64, char) = (1, 3.14, 'a');` | - | Ordered, fixed-size collection of elements of different types |
| Slice         | `let slice: &[i32] = &[1, 2, 3];` | - | Reference to a contiguous sequence of elements |
| String        | `let string: String = String::from("hello");` | - | Dynamic, growable UTF-8 encoded string |
| Vec           | `let vec: Vec<i32> = vec![1, 2, 3];` | - | Dynamic, growable, heap-allocated vector |
| HashMap       | `use std::collections::HashMap;`<br>`let mut map = HashMap::new();`<br>`map.insert("key", "value");` | - | Collection of key-value pairs with a flexible size |
| Option        | `let option: Option<i32> = Some(42);` | - | Represents an optional value, either `Some(value)` or `None` |
| Result        | `let result: Result<i32, &str> = Ok(42);` | - | Represents a computation that can result in either success (`Ok(value)`) or failure (`Err(reason)`) |
| Enum          | `enum Shape { Circle(f64), Rectangle(f64, f64) }`<br>`let circle = Shape::Circle(3.14);` | - | Defines a type with multiple possible values (variants) |


These are some common compound types in Rust. Arrays have a fixed size, tuples can have elements of different types, slices are references to a portion of an array, and strings are dynamically allocated and growable. Vectors (`Vec`) are similar to arrays but can change in size, and `HashMap` is a collection of key-value pairs. `Option` and `Result` are used for handling optional and error-prone situations, respectively. Enums allow you to define your own types with multiple possible values.
