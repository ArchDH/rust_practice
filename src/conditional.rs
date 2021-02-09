// Check the condition of code

pub fn run(){
    let age = 50;

    // if-else
    if age >= 20{
        println!("Adult is 50$");
    }
    else if age == 20{
        println!("20 is 30$")
    }
    else{
        println!("Kid is 10$");
    }

    // short-handed if-else
    let is_of_age = if age >=21 {true} else {false};
    println!("{}", is_of_age);
} 