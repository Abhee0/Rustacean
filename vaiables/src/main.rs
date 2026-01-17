// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);

//     const PI : i32 = 3.14;
//     println!("The value of PI is: {PI}");
// }

// const ABHEE : i32 = 10;    

fn main() {
    let x = 15;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

fn another_function (){

    // i8 range is from -128 to 127
    //-(2n - 1 ) to 2n -1 -1
    let a : i8 = 5;
}