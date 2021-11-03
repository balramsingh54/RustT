fn main() {
    loop {
        println!("continue printing...");
    }

    //for temp_var in lower value.. max value;

    for x in 1..11{
        if x==5{
            continue;
        }
        println!("the value of x is : {}",x);
    }

    //loop is also called while loop 
    let mut i= 1;
    loop {
        println!("this is the value of i {}",i);
        if i==5{
            break;
        }
        i=i+1;
    }

    // 1 2 3 4 5 6 7 8 9 10 using for loop

    let _i=0;
    for i in 1..11{
        print!{"{} ",i};
    }
    println!("");

    // 1 2 3 4 5 6 7 8 9 10 using while loop

    let mut j= 1;
    while j<11{
        print!{"{} ",j};
        j=j+1;
    }
    println!("");


    // print all value of array 

    let arr = [10,20,30,40,50];
    // let _i=0;
    for i in 0..5{
        print!("{} ",arr[i])
    
    }
    println!("");

}
