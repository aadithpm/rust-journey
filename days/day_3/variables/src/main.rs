use std::io;

fn main() {
    // primitive types
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let guess: i32 = "42".parse().expect("Not a number!");
    println!("{guess}");
    
    // compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (a, b, c) = tup;
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("The value of tup is {a}, {b}, {c}");

    // arrays cannot shrink or grow in size
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");
    let ret = another_function(5);
    println!("{ret}");
}

fn another_function(x: i32) -> i32 {
    return x + 10;
}
