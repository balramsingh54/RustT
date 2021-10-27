fn main() {
    
    let no = 3;
    if no<5{
        println!("condition is true");
    }
    else{
        println!("condition is false");
    }

    // else if condition
    let number = 6;
    if number % 4 ==0{
        println!("this number is divisible by 4");
    }
    else if number % 3==0{
        println!("this number is divisible by 3");
    }
    else if number % 2==0{
        println!("this number is divisible by 2");
    }
    else{
        println!("this number is not divisible by 4, 3, 2 ");
    }


    // using let in if condition

    let check = true;
    let p= if check {5} else {6};

    println!{"this is p {}", p};

}
