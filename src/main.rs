#[derive(Debug)]
struct Person {
    name: String,
    age: i8,
    email: String
}


fn main() {
    
    let person = Person{
        name: String::from("snowin"),
        age: 18,
        email: String::from("snowin@gmail.com"),
    };

    // Let us print the person
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Email: {}\n", person.email);
    
    // Let us destruct the preson
    let Person {name, age, email} = person;
    
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