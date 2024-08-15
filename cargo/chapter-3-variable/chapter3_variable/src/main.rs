// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

fn main(){

    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("let x be the value of inner function: {x}")
    }

    println!("value of the outer function {x}")


}



