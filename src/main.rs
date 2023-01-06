fn main() {
    /* Scalar Types */
    let age:i8 = 18;
    let cgp:f32 = 7.5;

    let is_active:bool = true;

    let initial:char = 'J';

    let name:&str = "snowin";

    println!("{} {} is {} years old with the CGP of {}. Active:{}.", name, initial, age, cgp, is_active);

    // compound data types
    let names:[&str; 3] = ["snowin", "mark", "kevin"];
    println!("Names: {:?}", names);   
}