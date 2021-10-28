// ownership and functions 

fn take_ownership(s:String){
    println!("{}",s);
}

fn move_copy(c:char){
    println!("{}",c);
}


fn main() {
    let st=String::from("Balram");
    take_ownership(st);

    let ch = 'a';
    move_copy(ch);
    println!("{}",ch);

    // copy string 
    let s1= String::from("BalramSingh");
    let s2=s1;
    println("{}",s2);
    
}
