/*
Integers : u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats : f32, f64
Boolean (bool)
char
Tuples
Arrays (vectors)
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer that type we want to use based on the value and how we use it. (like var in cs)
pub fn run(){
    // Defult is i32
    let x = 1;
    let y = 10.5;
    let z = true;
    let em = '\u{1F600}';

    println!("{} and {} and {}", x,y,z);

    // Find max size
    println!("Max i32 = {}", std::i32::MAX);
    println!("Max u32 = {}", std::u32::MAX);

    // debug place holder :?
    println!("{:?}", (x,y,z,em));

    // Boolean expression
    let boex : bool = 10 < 5 ;

}