// Tuples group togheter values of differnet types unlike arrays

pub fn run(){
    println!("Tuple Practice");

    let t:(&str, &str, i32) = ("Test", "test", 10);

    println!("{} and {} and {}", t.0, t.1, t.2);
}