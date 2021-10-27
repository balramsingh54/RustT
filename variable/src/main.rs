fn main() {

    // let mut x= 5;
    // println!("the value of x is {}", x);
    
    // x= 6;
    // println!("the value of x is {}", x);

    // let x= 7;
    // println!("the value of x is {}", x);


    // floating variables
    let x= 5.0;
    println!("{}",x); // this is f32 bit

    let y:f32 = 5.5;
    println!("{}",y);

    let z= 3.3;
    println!("{}",z);

    let a = "balram";
    println!("{}",a);

    let b:&str = "balram";
    println!("{}",b);


    // boolean type
     let bc= true;

     println!("{}",bc);

     let abc:bool= false;

     println!("{}",abc);

     // character types 

     let gf= 'a';
     println!("{}",gf);

    // tupple : it is way of grouping of numbers with different data types 
    let tup: (i32, f32, u32)= (22, 3.3, 3);
    let (x, y, z) = tup;
    println!("the value of x y and z is  {} {} {}",x, y, z);


    // Array type

    let aa = [1,2,3,4,5];
    // accessing array element

    let _fir = aa[0];
    let _sec = aa[1];
    println!("{}",_fir);
    println!("{}",_sec);
    

}
