// fn main() {
//     let my_name = "kunle";
//     my_name.push("!");
//     println!("{}", my_name);
// }

// fn main() {
//     let mut 
// }

fn main() {
    let mut my_name: String = "Kunle".to_string();
    my_name.push('!');
    println!("{}", my_name);
}

fn main() {
    // Rust program starts with a function defined fn
    /* println!("{}");*/ 
}

fn main() {
    let some_number = 100; // number of a value
    // a little about number
    // it's 100, which is my favouritte number
    // it's called some_number. but actually I think that it's

    let some_number = 100; /* let me tell you
    a little about this number.
    It is 100, which is my favouritte number.*/
}

fn main() {
    let first_letter = 'A';
    let space = ' '; // A space inside ' ' is also clear
    let other_language_char = ' '; // Thanks to unicode, other_language_char
    let cat_face = ' '; // Emoji are character entities
}

fn main() {
println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes 
println!("Size of string containing 'a': {}", "a".len()); // .len() giv 
println!("Size of string containing 'ß': {}", "ß".len()); 
println!("Size of string containing '国': {}", "国".len()); 
println!("Size of string containing '􏰂': {}", "􏰂".len());
}

fn main() {
    let slice = "Hello";
    println!("slice is {} byte", slice.len());
    let slice2 = "안녕!"; // Korean for "hi"
    println!("Slice is {} byte", slice2.len());
}

// .len() give size in bytes
fn main() {
    let slice = "hello";
    println!("Slice is {} byte and also {} characters", slice.len());
    let slice2 = "안녕!";
    println!("Slice is {} bytes but only {} characters",  slice2.len());
}

// Interference in Rust
fn main() {
    let small_slice: u8 = 10;
}

fn main() {
    let small_number = 10u8; // 10u8 = 10 of type u8
}

fn main() {
    let small_number = 10_u8; // 10_u8 = 10 of type u8
    let big_number = 100_000_000_i32; // 100 million is easy to read
}

fn main() {
    let my_float = 5.; // Rust see . and know that it is a float
}

fn main() {
    let my_float: f64 = 5.; // This is an f64
    let my_other_float: f32 = 5.; // This an f32
    let third_float = my_float + my_other_float; // error for mismatch because both are different float type
}

fn main() {
    let my_float: f64 = 5.0;
    let my_other_float: f32 = 8.5;
    let third_float = my_float + my_other_float as f64; // my_other_float is now made a float 64 type
}

// fn define a function
// main is the function that start the program
//() we did not pass the function to to an argument
//{} is the code block. This are where the code lives

fn main() {
    let my_float = 5.0;
    {
        let my_other_float = 8.5
    }
    // third_float = my_float + my_other_float;
}

fn number() -> i32 {
    8
}

fn main() {
    println!("Hello, world {}!", number());
}

fn multiply(number_one: i32, number_two: i32) -> i32 { // Two i32 will
    let result = number_one * number_two;
    println!("{} times is {}", number_one, number_two, result)
}

fn main(){
    multiply(8, 9);
    let some_num = 10;
    let some_other_num = 2;
    multiply(some_num, some_other_num);
}

fn multiply(number_one:i32, number_two:i32) -> i32 {
    let result = number_one * number_two;
    println!("{} times {}", number_one, number_two, result)
    result


// commenting in Rust might look confusing.

/* */

// Signed and Unsighed

//  i8, i16, i64, i128, and i256

// You saw that a char is always one character, and uses '' (single quotes) instead of "" (double quotes)

fn main() {
    println!("{:?}", "a".as_bytes())
    println!("{:?}", "b".as_bytes())
}

// Now if .len() gives the size in bytes, what about the size in characters? We will learn about these methods later, but for now you can just remember that .chars().count() will do it. .chars().count() turns what you wrote into characters and then counts how many there are.


fn main() {
    let slice = "Hello";
    println!("Slice is {} byte and also {} characters.", slice.1)
    let slice2 = "%^@^";
    println!("Slice2 is {} bytes and also {} characters.", slice.2)
}

fn main() {
    let my_num = 10u8; // 10 of type u8
}

fn main() {
    let my_large_num = 100_000_000_i32; // _ does not change the value of the number
}


fn main() {
    let my_flaot_num: f64 = 5.0;
    let other_num: f32 = 8.5

