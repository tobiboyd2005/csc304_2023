#![allow(dead_code)]

pub enum comp{
    LessThan,
    GreaterThan,
    Equal
}

pub enum Gender{
    Male,
    Female
}


#[derive(Debug)]
struct Person{
    name: String,
    age: u8
}

println!("{:?}");

struct Person2{
    name: &str,
    age: u8
}

pub fn run(){
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,};
    println!("{:?}",person);
}