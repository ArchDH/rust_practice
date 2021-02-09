// std = Immutable fixed-length string
// String = Groawble, heap-allocated data structure

pub fn run(){
    let hello1 = "Hello";
    let mut hello2 = String::from("Hello");

    // Get length
    println!("length {}", hello1.len());
    println!("length {}", hello2.len());

    hello2.push('w');
    println!("length {}", hello2.len());
    hello2.push_str("orld !");
    println!("length {}", hello2.len());

    // Capacity in bytes
    println!("Capacity = {}", hello2.capacity());
    
    // Empty
    println!("IS empty = {}", hello2.is_empty());
    
    // Contains (case-sensitive)
    println!("Contains 'world' = {}", hello2.contains("world"));

    // replace
    println!("Replace : {}", hello2.replace("world", "there"));
 
    // Loop through string by whitespace
    for word in hello2.split_whitespace(){
        println!("{}", word);
    }

    // Create a string with capacity
    let mut s = String::with_capacity(10);
    println!("is empty = {}", s.is_empty());
    s.push('a');
    s.push('b');


    // Asssetion testing
    assert_eq!(2, s.len());
    println!("{}", s);
}