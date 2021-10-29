//Borrowing allows to have multiple references to a single
// resource but still obeys to have a "single owner".

fn calculate_length(s:&String)-> usize{
    s.len()
}

fn change(strn: &mut String){
    strn.push_str(" world");
}

fn main() {
    let mut st= String::from("Balram");
    let len= calculate_length(&st);
    println!("size of string is {}", len);

    let mut ss:String = String::from("Hello");
    println!(" Before modification {}",ss);
    change(&mut ss);
    println!("{}",ss);

}
