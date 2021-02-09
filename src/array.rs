// Arrays = fixed list where elements are the same data types

pub fn run(){
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];  // let name: [type : length] = values
    println!("{:?}",numbers);
    
    // get single value
    println!("{}", numbers[0]);

    // Get values
    for i in numbers.iter() {
        println!("{}", i);
    }
    
    // Get array length
    println!("array length = {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes ({} bites) on stack", std::mem::size_of_val(&numbers), std::mem::size_of_val(&numbers)*8);

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice : {:?}", slice);
}