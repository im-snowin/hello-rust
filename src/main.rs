fn main() {
    
    let mut years: [i32; 3] = [1920, 1947, 2002];

    println!("{:?}", years);
    
    years[2] = 1988;
    
    println!("{:?}", years);

    for year in years.iter() {
        println!("Next year: {}", year + 1);
    }

    let names: [&str; 4] = ["snowin", "mark", "kalo", "john"];
    println!("{:?}", names)
}