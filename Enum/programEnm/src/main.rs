#[derive(Debug)]

enum Employee{
    Name(String),
    Id(i32),
    Profile(String),
}

fn main() {
    let name= Employee::Name("Balram".to_string()); // for instance print :? is used 
    let id = Employee::Id(1);
    let profile = Employee::Profile("Computer Science".to_string());
    println!("{:?} {:?} {:?}", name, id, profile);
}
