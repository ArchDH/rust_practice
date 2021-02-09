pub fn run(){
    // print to console
    println!("Println file");
    println!("{} and {}", "Place Holder 1", "Place Holder 2");
    println!("{0} and {1} and {1}", "PH1", "PH2");

    // Variables
    let var1 = "Test";
    let mut var2 = 10;
    println!("var1 is {} and var2 is {}", var1, var2);
    var2 = 50;
    println!("now var2 is {}", var2);

    // Define constant (type must be defined)
    const ID: i32 = 001; // int 32
    println!("ID = {}", ID);

    // Multiple variables
    let (var3 , var4) = ("Var3", "Var4");
    println!("{} and {}", var3, var4);
}