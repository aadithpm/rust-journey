fn main() {
    let mut v1: Vec<i32> = Vec::new();

    v1.push(5);
    v1.push(6);

    let mut v2 = vec![1, 2, 3];

    for i in &mut v2 {
        *i += 50;
    }

    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v1.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // This code below _should_ technically work, with how I understand Rust works so far:
    //
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");
    // 
    // However, it fails because of the existence of a mutable and an immutable reference in the same scope.
    // &v is an immutable reference and then we try to use a mutable reference 'v'
    // The compiler doesn't let you do this because:
    // - pushing an element might require additional memory, and the old values will need to be copied to the new space
    // - if copying is required (this will happen if there is not enough memory next to each other), this will cause the 
    // immutable reference to point to deallocated memory

    // a vector can hold different types, if we use an enum
    enum SpreadsheetCell {
        Int(i32),
        Text(String),
        Float(f32),
    }

    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(5.0),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Integer: {}", i),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
        }
    }


    // Strings
    // Strings in rust are implemented as a wrapper around vectors
    let data = "initial contents";
    let s = data.to_string();
    // The method also works on a literal directly:
    let s = "initial contents".to_string();
    // We can also use the function String::from to create a String from a string literal
    let mut s = String::from("initial contents");
    let s2 = "bar";
    s.push_str(s2);
    println!("s is {s}");

    // Hashmaps
    // Just like vectors, hash maps store their data on the heap
    // This HashMap has keys of type String and values of type i32
    // Like vectors, hash maps are homogeneous: all of the keys must have the same type, 
    // and all of the values must have the same type
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // upsert
    scores.entry(String::from("Yellow")).or_insert(50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // For types that implement the Copy trait, like i32, the values are copied into the hash map
    // For owned values like String, the values will be moved 
    // and the hash map will be the owner of those values
}
