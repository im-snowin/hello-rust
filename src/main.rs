#[derive(Debug)]
struct Person {
    name: String,
    age: i8,
    email: String
}


fn main() {
    
    let person1: Person= Person{
        name: String::from("snowin"),
        age: 18,
        email: String::from("snowin@gmail.com"),
    };

    // Let us print the person1
    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
    println!("Email: {}\n", person1.email);
    
    // Let us destruct the preson
    let Person {name, age, email} = person1;
    
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Email: {}\n", email);

    // lets mut struct
    let mut person2 = Person{
        name: String::from("mark"),
        age: 23,
        email: String::from("mark@gmail.com"),
    };

    person2.name = String::from("Mark Henry");

    println!("{:#?}", person2)

}