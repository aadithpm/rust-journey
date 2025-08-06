fn main() {
    // sl is a 'string literal' and is hardcoded into the application, is faster
    // at compile time. the speed and efficiency are a property of immutability
    let sl = "hello world";
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{sl}");
    println!("{s}");
    // Rust's memory management automatically frees memory used for sl and s 
    // at the end of a block. Their 'scope' is only this block
    let s1 = String::from("hello");
    let s2 = s1;
    // you _cannot_ use s1 here since Rust s1 as no longer valid. Rust does something
    // called a 'move', a version of a shallow copy that also invalidates what is being copied.
    // so in this case, s1 is invalidated after it's 'moved' to s2 i.e the underlying pointer
    // to the heap is moved over to s2. this is because a String uses heap memory so the size of
    // that variable is unknown at compile time, which makes it risky.
    // you can use s1.clone() instead if you want to deep copy
    
    println!("{s2}, world!");

    // ..but there's also the 'Copy' trait 
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
    // unlike the above {s1, s2} scenario, both x _and_ y can be used here.
    // this is because integers use stack memory, have a known size so copies can be made quickly.
    // this is achieved because these types (integer for example) have a 'Copy' trait. 
    // primitive types implement the 'Copy' trait and so do tuples as long as they consist of types
    // that implement the 'Copy' trait (note that 'tuples' in Rust can consist of different types)
    // whew

    // rust also 'passes' ownership to functions like so:
    let bar = 10;
    let hex = String::from("hex");
    foo(hex, bar);
    // can't use 'hex' here since the ownership has been 'passed' to the function so it's no longer valid
    // in this scope. However, 'bar' is still valid because i32 implements the 'Copy' trait.
    // same principle applies to returning values from functions, the ownership is 'passed'.
    //
    // but there's more! you don't have to pass ownership if you pass a reference
    // this concept is called 'borrowing' in Rust. The scope of ref_str and the ownership is still in main
    // and it's freed when main ends, not in by_ref().
    // note that borrowing is immutable by default, by_ref cannot modify ref_str, unless a mutable reference is used.
    let mut ref_str = String::from("reference string");
    by_ref(&mut ref_str);
    println!("this is still accessible here: {ref_str}");
    // another note: you cannot pass multiple mutable references to the same data in memory, to protect against a data race
}

fn foo(hex: String, bar: i32) {
    println!("{hex} {bar}");
}

fn by_ref(s: &mut String) {
    s.push_str(", added in fn");
    println!("passed in a reference: {s}");
}