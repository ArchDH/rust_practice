// vectors = dynamic lists where elements are the same data types

pub fn run(){
    let mut numbers : Vec<i32> = vec![1, 2, 3, 4, 5];  // let mut name: Vec<type> = vec![values]

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

    // add elements
    numbers.push(10);
    numbers.push(20);

    println!("{:?}", numbers);

    // pop off the last value
    numbers.pop();
    println!("{:?}", numbers);

    // Loop throught vectur values
    for i in numbers.iter() {
        println!("number : {}", i);
    }

    // Loop and mutate values
    for i in numbers.iter_mut(){
        *i +=10;
    } 
    println!("{:?}", numbers);
}