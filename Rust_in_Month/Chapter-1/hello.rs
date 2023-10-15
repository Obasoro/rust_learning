fn main() {
    let my_num = 5;
    {
        let other_num = 9; 
    }
    println!("{} and {}", my_num, other_num);
}

fn number() -> i32 {
    8
}

fn main() {
    println!("hello, world number {}!", number());
}

fn multiply(number_one: i32, number_two: i32) { // two i32s will be multiplied
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result)

}

fn main() {
    multiply(8,9);
    let some_other_num = 10;
    let other_num = 3;
    multiply(some_other_num, other_num);
}


